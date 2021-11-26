fn main() {
    cc::Build::new().file("tweetnacl.c").compile("tweetnacl"); // outputs `libtweetnacl.a`
}
