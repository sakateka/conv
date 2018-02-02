
use std::char;
use std::io::{BufRead, Read, BufReader, BufWriter, Write};
use byteorder::{WriteBytesExt};
use std::ops::Deref;
use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/charmaps.rs"));

#[derive(Debug)]
pub struct Converter {
    from: String,
    to: Option<&'static HashMap<u32, u8>>,
}

#[derive(Debug)]
pub struct Decoder<R> {
    r: R,
    index: usize,
    buf_len: usize,
    buf: Vec<u32>,
    map: Option<&'static HashMap<u8, u32>>,
}

impl <R: Read + BufRead> Decoder<R> {
    fn new(r: R, code: &str) -> Decoder<R> {
        Decoder{
            r: r,
            index: 0,
            buf_len: 0,
            buf: Vec::new(),
            map: get_decode_map(code),
        }
    }
}

impl <R: Read + BufRead> Iterator for Decoder<R> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index < self.buf_len {
            return Some(self.buf[self.index]);
        }
        self.buf.truncate(0);
        let mut length = 0;
        let item = match self.r.fill_buf() {
            Ok(buf) => {
                length = buf.len();
                if length == 0 {
                    return None
                }
                match self.map {
                    Some(m) => {
                        for b in buf.iter() {
                            if let Some(x) = m.get(b) {
                                self.buf.push(*x);
                            }
                        }
                    },
                    None => {
                        let string = String::from_utf8_lossy(buf);
                        for c in string.chars() {
                            let key = c as u32;
                            if key < 0xffff_u32 {
                                self.buf.push(key);
                            } else {
                                self.buf.push(0xfffd);
                            };
                        }
                    },
                }
                self.buf_len = self.buf.len();
                self.index = 0;
                Some(self.buf[self.index])
            },
            Err(_) => None,
        };
        self.r.consume(length);
        item
    }
}

impl Converter {
    pub fn new(from_code: &str, to_code: &str) -> Converter {
        Converter {
            from: from_code.to_owned(),
            to: get_encode_map(to_code),
        }
    }
    pub fn convert<R: Read, W: Write>(&self, src: R, dst: W) where R: Sized {

        let one_mb = 1_usize << 20;
        let buf_src = BufReader::with_capacity(one_mb, src);
        let mut buf_dst = BufWriter::with_capacity(one_mb, dst);
        let decoder = Decoder::new(buf_src, &self.from);

        let encode = !self.to.is_none();
        let mut map = &HashMap::new();
        if encode {
            map = self.to.unwrap();
        }
        for key in decoder {
            if encode {
                match map.get(&key) {
                    Some(x) => buf_dst.write_u8(*x).unwrap(),
                    None => buf_dst.write_u8(b'?').unwrap(),
                };
            }else {
                buf_dst.write_fmt(format_args!("{}", char::from_u32(key).unwrap())).unwrap();
            }
        }
    }
}
