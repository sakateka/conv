use std::env;
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("charmaps.rs");
    let mut charmaps = File::create(&dest_path).unwrap();

    let mut char_tables = Vec::new();

    let paths = fs::read_dir("charmaps").expect("read charmaps/*");

    for path in paths {
        let p = path.unwrap().path();
        let name = p.display().to_string();
        let key = name.splitn(2, '.').nth(0).unwrap()
            .splitn(2, '/').nth(1).unwrap().to_uppercase().replace('-', "");

        let mut content = String::new();
        let mut f = File::open(p).expect(&format!("Open charmap {:?}", name));
        f.read_to_string(&mut content).expect("read charmap");

        build_map(key.as_ref(), "utf8", &content, &mut charmaps);
        build_map("utf8", key.as_ref(), &content, &mut charmaps);
        char_tables.push(key);
    }
    build_encode_map_selector(&char_tables, &mut charmaps);
    build_decode_map_selector(&char_tables, &mut charmaps);

}

fn build_map(from: &str, to: &str, content: &String, out: &mut File) {
    let (size_from, size_to) = if from == "utf8" { ("u32", "u8") } else { ("u8", "u32") };
    out.write_fmt(format_args!(
        "lazy_static! {{\nstatic ref {}_TO_{}: HashMap<{}, {}> = [",
        from.to_uppercase(), to.to_uppercase(), size_from, size_to
    )).unwrap();

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
        let utf8_opt = fields.next();
        if from == "utf8" {
            match utf8_opt {
                Some(x) => {
                    out.write_fmt(format_args!("({}, {}), ", x, code)).unwrap();
                },
                _ => (),
            };
        } else {
            let x = utf8_opt.unwrap_or("0xfffd");
            out.write_fmt(format_args!("({}, {}), ", code, x)).unwrap();
        }
    }

    out.write(b"].iter().cloned().collect();\n}\n").unwrap();
}

fn build_decode_map_selector(tables: &Vec<String>, out: &mut File) {
    out.write(b"\n\
        fn get_decode_map(from: &str) -> Option<&'static HashMap<u8, u32>> {\n\
            \tmatch from.to_uppercase().as_ref() {\n\
    ").unwrap();

    for k in tables {
        out.write_fmt(format_args!(
            "\t\t\"{}\" => Some({}_TO_UTF8.deref()),\n", k, k.to_uppercase()
        )).unwrap();
    }

    out.write(b"\n\t\t\"UTF8\"=> None,\n").unwrap();
    out.write(b"\n\t\t_ => unimplemented!(\
            \"decode from {} not supported yet!\", from\
        ),\n\t}\n}"
    ).unwrap();
}

fn build_encode_map_selector(tables: &Vec<String>, out: &mut File) {
    out.write(b"\n\
        fn get_encode_map(to: &str) -> Option<&'static HashMap<u32, u8>> {\n\
            \tmatch to.to_uppercase().as_ref() {\n\
    ").unwrap();

    for k in tables {
        out.write_fmt(format_args!(
            "\t\t\"{}\" => Some(UTF8_TO_{}.deref()),\n", k, k.to_uppercase()
        )).unwrap();
    }

    out.write(b"\n\t\t\"UTF8\"=> None,\n").unwrap();
    out.write(b"\n\t\t_ => unimplemented!(\
            \"encode to {} not supported yet!\", to\
        ),\n\t}\n}"
    ).unwrap();
}
