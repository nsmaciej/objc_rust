extern crate gcc;

fn main() {
    gcc::Config::new().file("objc/sample.m").flag("-fmodules").compile("libsample.a");
}
