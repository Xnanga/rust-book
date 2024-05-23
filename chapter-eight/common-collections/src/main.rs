pub mod vectors;
pub mod strings;
pub mod hashmaps;

fn main() {
    crate::vectors::run_vectors();
    crate::strings::run_strings();
    crate::hashmaps::run_hashmaps();
}