extern crate clap;
mod args;
mod conv;
mod charmap;

fn main() {
    let app = args::build_app("conv");
    println!("Hello, world! {:?}", app);
}
