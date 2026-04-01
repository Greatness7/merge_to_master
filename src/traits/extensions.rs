use std::ffi::OsStr;

use tes3::esp::{Header, ObjectInfo};

use crate::prelude::*;

#[ext(HeaderExt)]
pub impl Header {
    /// Ensure the file name from `master_path` is present in the masters list.
    ///
    /// If the name was not present it will be inserted at the end of the list.
    ///
    fn ensure_master_present<'a>(&mut self, master_path: &'a Path) -> Result<&'a str> {
        let Some(master_name) = master_path.file_name().and_then(OsStr::to_str) else {
            bail!("Invalid master path.");
        };

        let master_position = self
            .masters
            .iter()
            .position(|(name, _)| name.eq_ignore_ascii_case(master_name));

        match master_position {
            Some(i) if i != (self.masters.len() - 1) => {
                bail!("Merge target must be the last master in plugin's master list.");
            }
            None => {
                self.masters.push((master_name.into(), master_path.metadata()?.len()));
            }
            _ => {
                // The master is present and is the last in the list, nothing to do.
            }
        }

        Ok(master_name)
    }

    fn collect_masters(plugin_paths: &[PathBuf]) -> Result<Vec<(String, u64)>> {
        plugin_paths
            .iter()
            .map(|path| {
                let file_size = path.metadata()?.len();
                let file_name = path
                    .file_name()
                    .context("Path has no file name")?
                    .to_str()
                    .context("Path has invalid UTF-8")?
                    .to_owned();
                Ok((file_name, file_size))
            })
            .collect()
    }

    fn build_master_remap(masters: &[(String, u64)]) -> HashMap<&UncasedStr, u32> {
        masters
            .iter()
            .enumerate()
            .map(|(index, (name, _))| (name.as_uncased(), (index + 1) as u32))
            .collect()
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
