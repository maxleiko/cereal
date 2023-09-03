use std::io::{self, Write};

pub trait Serialize {
    fn serialize<W>(&self, bytes: W) -> io::Result<usize>
    where
        W: Write;
}

macro_rules! serialize_impl {
    ($ty:ident) => {
        impl Serialize for $ty {
            fn serialize<W>(&self, mut bytes: W) -> io::Result<usize>
            where
                W: Write,
            {
                let buf = self.to_le_bytes();
                bytes.write_all(&buf)?;
                Ok(buf.len())
            }
        }
    };
}

macro_rules! serialize_varint_impl {
    ($ty:ident) => {
        impl Serialize for $ty {
            fn serialize<W>(&self, mut bytes: W) -> io::Result<usize>
            where
                W: Write,
            {
                integer_encoding::VarIntWriter::write_varint(&mut bytes, *self)
            }
        }
    };
}

serialize_impl!(i8);
serialize_impl!(u8);
serialize_impl!(f32);
serialize_impl!(f64);

serialize_varint_impl!(i16);
serialize_varint_impl!(u16);
serialize_varint_impl!(i32);
serialize_varint_impl!(u32);
serialize_varint_impl!(i64);
serialize_varint_impl!(u64);
serialize_varint_impl!(usize);
serialize_varint_impl!(isize);

impl Serialize for bool {
    fn serialize<W>(&self, mut bytes: W) -> io::Result<usize>
    where
        W: Write,
    {
        let byte = if *self { 1u8 } else { 0u8 };
        bytes.write_all(&[byte])?;
        Ok(1)
    }
}

impl Serialize for str {
    fn serialize<W>(&self, mut bytes: W) -> io::Result<usize>
    where
        W: Write,
    {
        let str_bytes = self.as_bytes();
        let str_len = str_bytes.len();
        let mut n = integer_encoding::VarIntWriter::write_varint(&mut bytes, str_len as u32)?;
        bytes.write_all(str_bytes)?;
        n += str_len;
        Ok(n)
    }
}

impl Serialize for &str {
    #[inline]
    fn serialize<W>(&self, bytes: W) -> io::Result<usize>
    where
        W: Write,
    {
        Serialize::serialize(*self, bytes)
    }
}

impl Serialize for String {
    #[inline]
    fn serialize<W>(&self, bytes: W) -> io::Result<usize>
    where
        W: Write,
    {
        Serialize::serialize(self.as_str(), bytes)
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize<W>(&self, mut bytes: W) -> io::Result<usize>
    where
        W: Write,
    {
        let mut n = self.len().serialize(&mut bytes)?;
        for elem in self {
            n += elem.serialize(&mut bytes)?;
        }
        Ok(n)
    }
}
