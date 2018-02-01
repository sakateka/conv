extern crate clap;

#[macro_use]
extern crate lazy_static;

mod args;
mod conv;
//mod charmap;

fn main() {
    let app = args::build_app("conv");
    let input = app.value_of("SOURCE").unwrap_or_else(|| {"/dev/stdin"});
    let output = app.value_of("output").unwrap_or_else(|| {"/dev/stdout"});
    let safely = app.is_present("safely");

    println!("Hello, world! {} -> {} : {}", input, output, safely);
}
