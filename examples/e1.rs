extern crate link_test;

use link_test::call_c;

fn main() {
    println!("example got {}", call_c());
}
