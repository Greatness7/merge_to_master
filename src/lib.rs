mod logging;
mod merge_objects;
mod merge_plugins;
mod plugin_data;
mod remap_masters;
mod remap_textures;

pub use logging::*;
pub use merge_objects::*;
pub use merge_plugins::*;
pub use plugin_data::*;
pub use remap_masters::*;
pub use remap_textures::*;

pub mod prelude {
    pub use super::*;

    pub use std::borrow::Cow;
    pub use std::ffi::OsStr;
    pub use std::mem::{swap, take};
    pub use std::path::{Path, PathBuf};

    pub use anyhow::{bail, ensure, Context, Result};
    pub use easy_ext::ext;
    pub use hashbrown::{hash_map::Entry, HashMap, HashSet};
    pub use itertools::Itertools;

    pub use path_slash::*;
    pub use rayon::prelude::*;

    pub fn default<T: Default>() -> T {
        Default::default()
    }
}
