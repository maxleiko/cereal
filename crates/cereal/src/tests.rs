use crate as cereal;
use crate::*;

#[test]
fn foo() {
    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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

    let input = Foo {
        i8: -42,
        u8: 42,
        i16: -2,
        u16: 4,
        i32: -655957,
        u32: 655957,
        i64: -200_000_000,
        u64: 200_000_000,
    };
    let mut bytes = vec![];
    let n = input.serialize(&mut bytes).unwrap();
    assert_eq!(n, 19);
    let output = Foo::deserialize(&mut &*bytes).unwrap();
    assert_eq!(input, output);
}

#[test]
fn struct_with_string() {
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct Foo {
        name: String,
    }

    let input = Foo {
        name: "hello".to_owned(),
    };
    let mut bytes = vec![];
    let n = input.serialize(&mut bytes).unwrap();
    assert_eq!(n, 6);
    let output = Foo::deserialize(&mut &*bytes).unwrap();
    assert_eq!(input, output);
}

#[test]
fn manual_test() {
    let mut bytes = vec![];
    Serialize::serialize(&1337_i32, &mut bytes).unwrap();
    Serialize::serialize("hello world", &mut bytes).unwrap();
    let bytes = &mut bytes.as_slice();
    let first: i32 = Deserialize::deserialize(bytes).unwrap();
    let second: &str = Deserialize::deserialize(bytes).unwrap();
    assert_eq!(first, 1337);
    assert_eq!(second, "hello world");
}

#[test]
fn varint() {
    let mut bytes = vec![];
    let n = Serialize::serialize(&655957_i64, &mut bytes).unwrap();
    assert_eq!(n, 3);
    let bytes = &mut &*bytes;
    let result: i64 = Deserialize::deserialize(bytes).unwrap();
    assert_eq!(result, 655957);
}

#[test]
fn generic() {
    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
    struct Foo<T> {
        field: T,
    }

    let input = Foo {
        field: "hello world",
    };
    let mut bytes = vec![];
    let n = input.serialize(&mut bytes).unwrap();
    assert_eq!(n, 12);
    let output = Foo::deserialize(&mut &*bytes).unwrap();
    assert_eq!(input, output);
}
