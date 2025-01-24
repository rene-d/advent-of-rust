fn main() {
    // Buy a Mac 
    println!(r"cargo:rustc-link-search=/opt/homebrew/lib");

    println!("cargo:rerun-if-changed=src/year2018/day9_c/day9.c");
    cc::Build::new()
        .file("src/year2018/day9_c/day9.c")
        .compile("day9_c");
}
