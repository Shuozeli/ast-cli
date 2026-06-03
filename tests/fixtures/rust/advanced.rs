use std::collections::HashMap;
use std::fmt;

// ── Type alias ──────────────────────────────────────────
pub type Result<T> = std::result::Result<T, Error>;

// ── Enum with data variants ─────────────────────────────
#[derive(Debug)]
pub enum Error {
    NotFound { key: String },
    ParseError(String),
    Internal,
}

// ── Union ───────────────────────────────────────────────
pub union IntOrFloat {
    i: i32,
    f: f32,
}

// ── Struct with lifetimes and generic bounds ────────────
pub struct Registry<'a, T: Clone + fmt::Debug> {
    entries: HashMap<&'a str, T>,
    capacity: usize,
}

// ── Trait with associated type and default method ───────
pub trait Transform {
    type Output;

    fn transform(&self) -> Self::Output;

    fn name(&self) -> &str {
        "unnamed"
    }
}

// ── Supertrait ──────────────────────────────────────────
pub trait Processor: Transform + fmt::Display {
    fn process(&mut self) -> Result<()>;
}

// ── Impl block with complex bounds ──────────────────────
impl<'a, T: Clone + fmt::Debug> Registry<'a, T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: HashMap::new(),
            capacity,
        }
    }

    pub fn insert(&mut self, key: &'a str, value: T) -> Result<()> {
        if self.entries.len() >= self.capacity {
            return Err(Error::Internal);
        }
        self.entries.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.entries.get(key)
    }
}

// ── Trait impl with where clause ────────────────────────
impl<'a, T> Transform for Registry<'a, T>
where
    T: Clone + fmt::Debug + Default,
{
    type Output = Vec<T>;

    fn transform(&self) -> Self::Output {
        self.entries.values().cloned().collect()
    }
}

// ── Display trait impl ──────────────────────────────────
impl<'a, T: Clone + fmt::Debug> fmt::Display for Registry<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Registry({} entries)", self.entries.len())
    }
}

// ── Async function ──────────────────────────────────────
pub async fn fetch_data(url: &str) -> Result<String> {
    let _ = url;
    Ok("data".to_string())
}

// ── Const function ──────────────────────────────────────
pub const fn max_capacity() -> usize {
    1024
}

// ── Unsafe function ─────────────────────────────────────
pub unsafe fn raw_pointer_deref(ptr: *const u8) -> u8 {
    *ptr
}

// ── Macro definition ────────────────────────────────────
macro_rules! define_wrapper {
    ($name:ident, $inner:ty) => {
        pub struct $name {
            inner: $inner,
        }
    };
}

// ── Nested module ───────────────────────────────────────
mod inner {
    pub struct InnerStruct {
        pub value: i32,
    }

    impl InnerStruct {
        pub fn new(value: i32) -> Self {
            Self { value }
        }
    }

    pub fn helper() -> bool {
        true
    }
}

// ── Static item ─────────────────────────────────────────
pub static DEFAULT_NAME: &str = "default";

// ── Const item ──────────────────────────────────────────
pub const VERSION: u32 = 1;

// ── Foreign mod ─────────────────────────────────────────
extern "C" {
    pub fn external_c_function(x: i32) -> i32;
    static EXTERNAL_STATIC: i32;
}

// ── Test module ─────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_insert() {
        let mut reg = Registry::new(10);
        assert!(reg.insert("a", 1).is_ok());
    }

    #[test]
    fn test_max_capacity() {
        assert_eq!(max_capacity(), 1024);
    }
}
