fn main() {
    cc::Build::new()
        .file("c_lib/average.c")
        .compile("average");
}