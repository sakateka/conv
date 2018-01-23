extern crate clap;
mod args;

fn main() {
    let app = args::build_app("conv");
    println!("Hello, world! {:?}", app);
}
