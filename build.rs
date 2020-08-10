fn main() {
    cc::Build::new()
        .include("depend")
        .file("depend/wrapper.c")
        .compile("wrapper");
}
