use cereal::{Deserialize, Readable, Writable};
use cereal_macro::Serialize;

#[derive(Readable, Writable, PartialEq, Eq, Debug)]
struct Foo {
    i8: i8,
    u8: u8,
    i16: i16,
    u16: u16,
    i32: i32,
    u32: u32,
    i64: i64,
    u64: u64,
}

fn main() {
    let input = Foo {
        i8: -42,
        u8: 42,
        i16: -1337,
        u16: 1337,
        i32: -655957,
        u32: 655957,
        i64: -200_000_000,
        u64: 200_000_000,
    };
    let mut bytes = vec![];
    let n = input.write(&mut bytes).unwrap();
    println!("n={n}");
    let output = Foo::from_bytes(&bytes).unwrap();
    assert_eq!(input, output);
}

#[allow(dead_code)]
fn manual_test() {
    let mut bytes = vec![];
    Serialize::serialize(&1337_i32, &mut bytes).unwrap();
    Serialize::serialize("hello world", &mut bytes).unwrap();
    let bytes = &mut bytes.as_slice();
    let first: i32 = Deserialize::deserialize(bytes).unwrap();
    let second: &str = Deserialize::deserialize(bytes).unwrap();

    println!("first={first}");
    println!("second={second}");
}
