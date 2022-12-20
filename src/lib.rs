#[cfg(feature = "conf")]
pub mod conf;
#[cfg(feature = "ini")]
pub mod ini;

#[cfg(feature = "json")]
pub mod json;

pub(crate) mod parser;
mod test;
