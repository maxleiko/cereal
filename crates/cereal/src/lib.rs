#[cfg(test)]
mod tests;

mod de;
mod ser;

pub use cereal_macro::*;
pub use de::*;
pub use ser::*;
