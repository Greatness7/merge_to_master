mod backup;
pub use backup::*;

mod logging;
pub use logging::*;

mod merge_plugins;
pub use merge_plugins::*;

mod traits;
pub use traits::*;

mod types;
pub use types::*;

pub mod prelude {
    pub use super::*;

    pub use std::path::{Path, PathBuf};

    pub use anyhow::{Context, Result, bail};
    pub use easy_ext::ext;
    pub use hashbrown::{HashMap, HashSet, hash_map::Entry};
    pub use itertools::Itertools;
    pub use uncased::AsUncased;

    pub use lending_iterator::prelude::*;
    pub use path_slash::*;
    pub use rayon::prelude::*;

    pub type UString = uncased::Uncased<'static>;

    pub fn default<T: Default>() -> T {
        Default::default()
    }
}
