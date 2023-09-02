use std::io;
use std::io::{Read, Write};

pub trait Writable {
    fn write(&self, bytes: &mut Vec<u8>) -> io::Result<usize>;
}

pub trait Readable {
    fn from_bytes(bytes: &[u8]) -> io::Result<Self>
    where
        Self: Sized;
}

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

deserialize_impl!(i8, 1);
deserialize_impl!(u8, 1);
deserialize_impl!(i16, 2);
deserialize_impl!(u16, 2);
deserialize_impl!(i32, 4);
deserialize_impl!(u32, 4);
deserialize_impl!(f32, 4);
deserialize_impl!(i64, 8);
deserialize_impl!(u64, 8);
deserialize_impl!(f64, 8);

impl<'de> Deserialize<'de> for String {
    fn deserialize(bytes: &mut &'de [u8]) -> io::Result<Self>
    where
        Self: Sized,
    {
        let len = u32::deserialize(bytes)? as usize;
        let mut buf = vec![0u8; len];
        bytes.read_exact(&mut buf)?;
        let string =
            String::from_utf8(buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        Ok(string)
    }
}

impl<'de> Deserialize<'de> for &'de str {
    fn deserialize(bytes: &mut &'de [u8]) -> io::Result<Self>
    where
        Self: Sized,
    {
        let len = u32::deserialize(bytes)? as usize;
        let str = std::str::from_utf8(&bytes[..len])
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        *bytes = &bytes[len..];
        Ok(str)
    }
}

impl<'de, T: Readable> Deserialize<'de> for T {
    fn deserialize(bytes: &mut &'de [u8]) -> io::Result<Self>
    where
        Self: Sized,
    {
        Self::from_bytes(bytes)
    }
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
