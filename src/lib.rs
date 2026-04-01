pub mod backup;
pub use backup::*;

pub mod logging;
pub use logging::*;

pub mod merge_plugins;
pub use merge_plugins::*;

pub mod traits;
pub use traits::*;

pub mod types;
pub use types::*;

pub mod prelude {
    pub use super::*;

    pub use std::path::{Path, PathBuf};

    pub use anyhow::{Context, Result, bail};
    pub use easy_ext::ext;
    pub use hashbrown::{HashMap, HashSet, hash_map::Entry};
    pub use itertools::{Either, Itertools};
    pub use smallvec::SmallVec;
    pub use uncased::{AsUncased, UncasedStr};

    pub use lending_iterator::prelude::*;
    pub use path_slash::*;
    pub use rayon::prelude::*;

    pub type UString = uncased::Uncased<'static>;

    pub fn default<T: Default>() -> T {
        Default::default()
    }
}
