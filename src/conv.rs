
use std::char;
use std::io::{BufRead, Read, BufReader, BufWriter, Write};
use byteorder::{WriteBytesExt};
use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/charmaps.rs"));

pub struct Decoder<R> {
    r: R,
    index: usize,
    buf_len: usize,
    buf: Vec<u32>,
    decode: bool,
    mapper: Box<Fn(usize) -> u32>,
}

impl <R: Read + BufRead> Decoder<R> {
    fn new(r: R, code: &str) -> Decoder<R> {
        Decoder{
            r: r,
            index: 0,
            buf_len: 0,
            buf: Vec::new(),
            decode: code != "utf8",
            mapper: get_decode_map(code),
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
                if self.decode {
                    for b in buf {
                        self.buf.push((self.mapper)(*b as usize));
                    }
                } else {
                    self.buf.extend(
                        String::from_utf8_lossy(buf).chars().map(|c| { c as u32 })
                    );
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

pub struct Converter {
    encode: bool,
    from: String,
    mapper: Box<Fn(u32) -> Option<&'static u8>>,
}

impl Converter {
    pub fn new(from_code: &str, to_code: &str) -> Converter {
        Converter {
            encode: &to_code[..] != "utf8",
            from: from_code.to_owned(),
            mapper: get_encode_map(to_code),
        }
    }
    pub fn convert<R: Read, W: Write>(&self, src: R, dst: W, safely: bool, replace: u8) where R: Sized {

        let one_mb = 1_usize << 20;
        let buf_src = BufReader::with_capacity(one_mb, src);
        let mut buf_dst = BufWriter::with_capacity(one_mb, dst);
        let decoder = Decoder::new(buf_src, &self.from);

        for key in decoder {
            if self.encode {
                buf_dst.write_u8(*(self.mapper)(key).unwrap_or({
                    if safely {
                        panic!("illegal input sequence '{:#x}'", key);
                    }
                    &replace
                })).unwrap();
            } else {
                buf_dst.write_fmt(format_args!("{}", char::from_u32(key).unwrap())).unwrap();
            }
        }
    }
}
