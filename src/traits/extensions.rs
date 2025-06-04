use std::ffi::OsStr;

use tes3::esp::{Header, ObjectInfo};

use crate::prelude::*;

#[ext(HeaderExt)]
impl Header {
    /// Ensure the file name from `master_path` is present in the masters list.
    ///
    /// If the name was not present it will be inserted at the end of the list.
    ///
    pub fn ensure_master_present<'a>(&mut self, master_path: &'a Path) -> Result<&'a str> {
        let Some(master_name) = master_path.file_name().and_then(OsStr::to_str) else {
            bail!("Invalid master path.");
        };

        let is_present = self
            .masters
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case(master_name));

        if !is_present {
            self.masters.push((master_name.into(), master_path.metadata()?.len()));
        }

        Ok(master_name)
    }
}

#[ext]
#[doc(hidden)]
impl<T> Option<T> {
    #[inline]
    pub fn map_into<U>(self) -> Option<U>
    where
        T: Into<U>,
    {
        self.map(T::into)
    }

    #[inline]
    pub fn set_ignored(&mut self, ignored: bool)
    where
        T: ObjectInfo,
    {
        if let Some(object) = self.as_mut() {
            object.set_ignored(ignored);
        }
    }
}

#[ext]
#[doc(hidden)]
impl<V: Default> HashMap<UString, V> {
    #[inline]
    pub fn get_or_insert_with<'a>(&'a mut self, key: &str, f: impl FnOnce() -> V) -> &'a mut V {
        let (_key, value) = self
            .raw_entry_mut()
            .from_key(key.as_uncased())
            .or_insert_with(|| (key.to_owned().into(), f()));
        value
    }

    #[inline]
    pub fn get_or_insert_default<'a>(&'a mut self, key: &str) -> &'a mut V
    where
        V: Default,
    {
        self.get_or_insert_with(key, default)
    }
}
