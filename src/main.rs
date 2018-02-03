extern crate clap;
extern crate byteorder;

#[macro_use]
extern crate lazy_static;

use std::fs::File;
mod args;
mod conv;
//mod charmap;

fn main() {
    let app = args::build_app("conv");
    let input = app.value_of("SOURCE").unwrap_or_else(|| {"/dev/stdin"});
    let output = app.value_of("output").unwrap_or_else(|| {"/dev/stdout"});
    let safely = app.is_present("safely");

    let from_code = app.value_of("from").unwrap();
    let to_code = app.value_of("to").unwrap();

    let converter = conv::Converter::new(from_code, to_code);
    let input_stream = File::open(input).unwrap();
    let mut output_stream = File::create(output).unwrap();
    let mut replace = b'?';
    if let Some(r) = app.value_of("replace") {
        replace = r.as_bytes()[0];
    }

    converter.convert(input_stream, &mut output_stream, safely, replace);
}
