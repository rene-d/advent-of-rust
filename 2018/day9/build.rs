fn main() {
    println!("cargo:rerun-if-changed=day9.c");

    cc::Build::new().file("day9.c").compile("day9");
}
