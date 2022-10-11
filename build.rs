use cc;

fn main() {
    cc::Build::new()
        .file("src/julian_day.c")
        .compile("libjd.a");
}
