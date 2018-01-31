use std::env;
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("charmaps.rs");
    let mut charmaps = File::create(&dest_path).unwrap();


    let paths = fs::read_dir("charmaps").expect("read charmaps/*");

    for path in paths {
        let p = path.unwrap().path();
        let name = p.display().to_string();
        let key = name.splitn(2, '.').nth(0).unwrap()
            .splitn(2, '/').nth(1).unwrap().to_lowercase().replace('-', "_");

        let mut f = File::open(p).expect(&format!("Open charmap {:?}", name));
        let mut content = String::new();
        f.read_to_string(&mut content).expect("read charmap");

        build_map(key.as_ref(), "unicode", &content, &mut charmaps);
        build_map("unicode", key.as_ref(), &content, &mut charmaps);
    }

}

fn build_map(from: &str, to: &str, content: &String, out: &mut File) {
    out.write_fmt(format_args!("let {}_to_{}: HashMap<u8, u16> = [", from, to)).unwrap();

    let mut num = 0;
    for line in content.lines() {
        if line.len() < 2 || &line[0..2] != "0x" {
            continue
        }
        if num % 5 == 0 {
            out.write(b"\n\t").unwrap();
        }
        num += 1;

        let codes: Vec<&str> = line.splitn(2, '#').collect();
        let mut fields = codes[0].split_whitespace();
        let code = fields.next().unwrap();
        let unicode = fields.next().unwrap_or("0xfffd");

        if from == "unicode" {
            out.write_fmt(format_args!("({}, {}), ", unicode, code)).unwrap();
        } else {
            out.write_fmt(format_args!("({}, {}), ", code, unicode)).unwrap();
        }
    }

    out.write(b"].iter().cloned().collect();\n").unwrap();
    out.write(b"\n").unwrap();
}
