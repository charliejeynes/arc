//! Set implementation.

pub mod inter_set;
pub mod light_set;
pub mod mat_set;
pub mod mesh_set;
pub mod react_set;
pub mod region_set;
pub mod spec_set;
pub mod state_set;
pub mod surf_set;

pub use self::{
    inter_set::*, light_set::*, mat_set::*, mesh_set::*, react_set::*, region_set::*, spec_set::*,
    state_set::*, surf_set::*,
};

use crate::{
    access,
    file::{as_json, from_json, Load, Save},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Display, path::Path};

/// Set mapping.
#[derive(Debug, Serialize, Deserialize)]
pub struct Set<K: Ord, T> {
    /// Internal map.
    map: BTreeMap<K, T>,
}

impl<K: Display + Ord, T> Set<K, T> {
    access!(map, BTreeMap<K, T>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(map: BTreeMap<K, T>) -> Self {
        Self { map }
    }

    /// Access a value within the map.
    #[inline]
    #[must_use]
    pub fn get(&self, key: &K) -> &T {
        self.map
            .get(key)
            .unwrap_or_else(|| panic!("Key {} does not exist within the set.", key))
    }

    /// Determine the index corresponding to a key within the set.
    #[inline]
    #[must_use]
    pub fn index_of_key(&self, key: &K) -> usize {
        for (i, k) in self.map.keys().enumerate() {
            if k == key {
                return i;
            }
        }

        panic!("Failed to find name within set.");
    }
}

impl<K: Display + Clone + Ord, T: Load> Set<K, T> {
    /// Load a set of files.
    #[inline]
    #[must_use]
    pub fn load(dir: &Path, keys: &[K], ext: &str) -> Self {
        let mut map = BTreeMap::new();

        for key in keys {
            let path = dir.join(format!("{}.{}", key, ext));
            println!("\tLoading: {}", path.display());

            map.insert((*key).clone(), T::load(&path));
        }

        Self::new(map)
    }
}

impl<K: Ord + Serialize, T: Serialize> Save for Set<K, T> {
    #[inline]
    fn save(&self, path: &Path) {
        as_json(self, path);
    }
}

impl<K, T> Load for Set<K, T>
where
    for<'de> K: Ord + Deserialize<'de>,
    for<'de> T: Deserialize<'de>,
{
    #[inline]
    #[must_use]
    fn load(path: &Path) -> Self {
        from_json(path)
    }
}
