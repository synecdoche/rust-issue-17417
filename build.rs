extern crate gcc;

fn main() {
    let mut build = gcc::Build::new();
    build.file("src/hello.c");
    build.compile("libhello.a");
}
