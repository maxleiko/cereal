use std::io::{self, Read};

pub trait Deserialize<'de> {
    fn deserialize(bytes: &mut &'de [u8]) -> io::Result<Self>
    where
        Self: Sized;
}

macro_rules! deserialize_impl {
    ($ty:ident, $size:expr) => {
        impl<'de> Deserialize<'de> for $ty {
            fn deserialize(bytes: &mut &'de [u8]) -> io::Result<Self>
            where
                Self: Sized,
            {
                let mut buf = [0u8; $size];
                bytes.read_exact(&mut buf)?;
                Ok($ty::from_le_bytes(buf))
            }
        }
    };
}

macro_rules! deserialize_varint_impl {
    ($ty:ident) => {
        impl<'de> Deserialize<'de> for $ty {
            fn deserialize(bytes: &mut &'de [u8]) -> io::Result<Self>
            where
                Self: Sized,
            {
                integer_encoding::VarIntReader::read_varint::<$ty>(bytes)
            }
        }
    };
}

deserialize_impl!(i8, 1);
deserialize_impl!(u8, 1);
deserialize_impl!(f32, 4);
deserialize_impl!(f64, 8);

deserialize_varint_impl!(i16);
deserialize_varint_impl!(u16);
deserialize_varint_impl!(i32);
deserialize_varint_impl!(u32);
deserialize_varint_impl!(i64);
deserialize_varint_impl!(u64);

impl<'de> Deserialize<'de> for String {
    fn deserialize(bytes: &mut &'de [u8]) -> io::Result<Self>
    where
        Self: Sized,
    {
        let str: &str = Deserialize::deserialize(bytes)?;
        Ok(str.to_owned())
    }
}

impl<'de> Deserialize<'de> for &'de str {
    fn deserialize(bytes: &mut &'de [u8]) -> io::Result<Self>
    where
        Self: Sized,
    {
        let len = integer_encoding::VarIntReader::read_varint::<u32>(bytes)? as usize;
        let str = std::str::from_utf8(&bytes[..len])
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        *bytes = &bytes[len..];
        Ok(str)
    }
}

impl<'de> Deserialize<'de> for bool {
    fn deserialize(bytes: &mut &'de [u8]) -> io::Result<Self>
    where
        Self: Sized,
    {
        if bytes.is_empty() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "reached EOF"));
        }
        let b = bytes[0] != 0;
        *bytes = &bytes[1..];
        Ok(b)
    }
}
