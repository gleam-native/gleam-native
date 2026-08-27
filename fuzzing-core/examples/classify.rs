//! Run with `cargo run -p fuzzing-core --example classify`
//!

fn main() {
    for src in [
        "pub fn main() { 1 }",
        "pub fn main() { let f = fn(x) { x + 1 }; f(41) }",
        "pub fn main() { 1 +. \"x\" }",
        "pub fn main() {",
    ] {
        println!("{src:?} -> {}", fuzzing_core::probe::probe_source(src));
    }
}
