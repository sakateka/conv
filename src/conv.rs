
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
    pub fn convert<R: Read, W: Write>(&self, src: R, dst: &mut W, safely: bool, replace: u8) where R: Sized {

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

#[cfg(test)]
mod tests {
    use self::super::*;
    use std::io::Cursor;

    #[test]
    fn koi8r_2_utf8() {
        let converter = Converter::new("koi8r", "utf8");
        // echo "Привет!"|iconv -f utf8 -t koi8r|hd
        let koi8r_hello = Cursor::new(vec![0xf0, 0xd2, 0xc9, 0xd7, 0xc5, 0xd4, b'!']);
        let mut output = Cursor::new(Vec::new());

        converter.convert(koi8r_hello, &mut output, false, b'?');
        assert_eq!(output.get_ref()[..], "Привет!".as_bytes()[..]);
    }

    #[test]
    fn koi8r_2_cp866() {
        let converter = Converter::new("koi8r", "cp866");
        // echo "Привет!"|iconv -f utf8 -t koi8r|hd
        let koi8r_hello = Cursor::new(vec![0xf0, 0xd2, 0xc9, 0xd7, 0xc5, 0xd4, b'!']);
        // echo "Привет!"|iconv -f utf8 -t cp866|hd
        let cp866_bytes = vec![0x8f, 0xe0, 0xa8, 0xa2, 0xa5, 0xe2, b'!'];
        let mut output = Cursor::new(Vec::new());

        converter.convert(koi8r_hello, &mut output, false, b'?');
        assert_eq!(output.get_ref()[..], cp866_bytes[..]);
    }
    #[test]
    fn utf8_2_cp1251() {
        let converter = Converter::new("utf8", "cp1251");
        let utf8_hello = Cursor::new("Привет!".as_bytes());
        // echo "Привет!"|iconv -f utf8 -t cp1251|hd
        let cp1251_bytes = vec![0xcf, 0xf0, 0xe8, 0xe2, 0xe5, 0xf2, b'!'];
        let mut output = Cursor::new(Vec::new());

        converter.convert(utf8_hello, &mut output, false, b'?');
        assert_eq!(output.get_ref()[..], cp1251_bytes[..]);
    }
}
