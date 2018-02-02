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

        let mut tab = parse_tab(&content);
        tab.sort_by(|a, b| {a.0.cmp(&b.0)});

        build_src_char_array(key.as_ref(), &tab, &mut charmaps);
        build_map(key.as_ref(), &tab, &mut charmaps);

        char_tables.push(key);
    }
    build_encoder_selector(&char_tables, &mut charmaps);
    build_decoder_selector(&char_tables, &mut charmaps);

}

fn parse_tab(content: &String) -> Vec<(String, Option<String>)> {
    let mut tab = Vec::new();
    for line in content.lines() {
        if line.len() < 2 || &line[0..2] != "0x" {
            continue
        }

        let codes: Vec<&str> = line.splitn(2, '#').collect();

        let mut fields = codes[0].split_whitespace();

        let key = fields.next().unwrap().to_owned();
        let code = match fields.next() {
            Some(c) => Some(c.to_owned()),
            None => None,
        };

        tab.push((key, code));
    }
    tab

}
fn build_src_char_array(code: &str,  tab: &Vec<(String, Option<String>)>, out: &mut File) {
    out.write_fmt(format_args!(
        "lazy_static! {{\n\tstatic ref {}_TO_UTF8: [u32;256] = [",
        code.to_uppercase(),
    )).unwrap();
    let mut num = 0;
    for &(_, ref rune) in tab {
        if num % 9 == 0 {
            out.write(b"\n\t\t").unwrap();
        }
        num += 1;
        let x = rune.clone().unwrap_or("0xfffd".to_owned());
        out.write_fmt(format_args!("{}, ", x)).unwrap();
    }
    out.write(b"\n\t];\n}\n").unwrap();
}

fn build_map(to: &str, tab: &Vec<(String, Option<String>)>, out: &mut File) {
    out.write_fmt(format_args!(
        "lazy_static! {{\n\tstatic ref UTF8_TO_{}: HashMap<u32, u8> = [",
        to.to_uppercase(),
    )).unwrap();

    let mut num = 0;
    for &(ref code, ref rune) in tab {
        if num % 5 == 0 {
            out.write(b"\n\t\t").unwrap();
        }
        num += 1;

        match rune {
            &Some(ref c) => {
                out.write_fmt(format_args!("({}, {}), ", c, code)).unwrap();
            },
            _ => (/* skip undefined chars */),
        };
    }
    out.write(b"\n\t].iter().cloned().collect();\n}\n").unwrap();
}

fn build_decoder_selector(tables: &Vec<String>, out: &mut File) {
    out.write(b"\n\
        fn get_decode_map(from: &str) -> Box<Fn(usize) -> u32> {\n\
            \tmatch from.to_uppercase().as_ref() {\n\
    ").unwrap();

    for k in tables {
        out.write_fmt(format_args!(
            "\t\t\"{}\" => Box::new(move |x| {{ {}_TO_UTF8[x] }}),\n", k, k.to_uppercase()
        )).unwrap();
    }

    out.write(b"\t\t\"UTF8\" => Box::new(move |x| {{ x as u32 }}),\n").unwrap();
    out.write(b"\
        \n\t\t_ => unimplemented!(\"decode from {} not supported yet!\", from),\n\
            \t}\n\
        }\n"
    ).unwrap();
}

fn build_encoder_selector(tables: &Vec<String>, out: &mut File) {
    out.write(b"\n\
        fn get_encode_map(to: &str) -> Box<Fn(u32) -> Option<&'static u8>> {\n\
            \tmatch to.to_uppercase().as_ref() {\n\
    ").unwrap();

    for k in tables {
        out.write_fmt(format_args!(
            "\t\t\"{}\" => Box::new(move |x| {{ UTF8_TO_{}.get(&x) }}),\n", k, k.to_uppercase()
        )).unwrap();
    }

    out.write(b"\t\t\"UTF8\" => Box::new(move |_x| {{ None /* unreachable */ }}),\n").unwrap();
    out.write(b"\
        \n\t\t_ => unimplemented!(\"encode from {} not supported yet!\", to),\n\
            \t}\n\
        }\n"
    ).unwrap();
}
