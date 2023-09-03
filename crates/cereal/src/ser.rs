use std::io::{self, Write};

pub trait Writable {
    fn write(&self, bytes: &mut Vec<u8>) -> std::io::Result<usize>;
}

pub trait Serialize {
    fn serialize(&self, bytes: &mut Vec<u8>) -> io::Result<usize>;
}

macro_rules! serialize_impl {
    ($ty:ident) => {
        impl Serialize for $ty {
            fn serialize(&self, bytes: &mut Vec<u8>) -> io::Result<usize> {
                let buf = self.to_le_bytes();
                bytes.write_all(&buf)?;
                Ok(buf.len())
            }
        }
    };
}

serialize_impl!(i8);
serialize_impl!(u8);
serialize_impl!(i16);
serialize_impl!(u16);
serialize_impl!(i32);
serialize_impl!(u32);
serialize_impl!(f32);
serialize_impl!(i64);
serialize_impl!(u64);
serialize_impl!(f64);

impl Serialize for str {
    fn serialize(&self, bytes: &mut Vec<u8>) -> io::Result<usize> {
        let str_bytes = self.as_bytes();
        let str_len = str_bytes.len() as u32;
        str_len.serialize(bytes)?;
        bytes.write_all(str_bytes)?;
        Ok(str_bytes.len() + 4)
    }
}

impl Serialize for &str {
    #[inline]
    fn serialize(&self, bytes: &mut Vec<u8>) -> io::Result<usize> {
        Serialize::serialize(*self, bytes)
    }
}

impl Serialize for String {
    #[inline]
    fn serialize(&self, bytes: &mut Vec<u8>) -> io::Result<usize> {
        Serialize::serialize(self.as_str(), bytes)
    }
}

impl<T: Writable> Serialize for T {
    #[inline]
    fn serialize(&self, bytes: &mut Vec<u8>) -> io::Result<usize> {
        self.write(bytes)
    }
}
