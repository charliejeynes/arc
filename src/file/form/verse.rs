//! Verse implementation.

use crate::{
    access,
    file::Surface as SurfaceForm,
    ord::{
        InterKey, InterSet, LightKey, LightSet, MatSet, MeshSet, ReactKey, ReactSet, RegionKey,
        RegionSet, Set, SpecSet, StateSet, SurfKey, SurfSet,
    },
    world::Verse as WorldVerse,
};
use attr::json;
use std::path::Path;

/// Verse construction form.
#[json]
pub struct Verse {
    /// List of interfaces.
    inters: Option<Vec<InterKey>>,
    /// List of regions.
    regions: Option<Vec<RegionKey>>,
    /// List of reactions.
    reacts: Option<Vec<ReactKey>>,
    /// List of lights.
    lights: Option<Vec<LightKey>>,
}

impl Verse {
    access!(inters, Option<Vec<InterKey>>);
    access!(regions, Option<Vec<RegionKey>>);
    access!(reacts, Option<Vec<ReactKey>>);
    access!(lights, Option<Vec<LightKey>>);

    /// Form a new instance.
    #[inline]
    #[must_use]
    pub fn form(&self, in_dir: &Path) -> WorldVerse {
        let mut inter_keys = if let Some(keys) = &self.inters {
            keys.clone()
        } else {
            Vec::new()
        };
        inter_keys.sort();
        inter_keys.dedup();
        let inters: InterSet = Set::load(&in_dir.join("interfaces"), &inter_keys, "json");

        let mut region_keys = if let Some(keys) = &self.regions {
            keys.clone()
        } else {
            Vec::new()
        };
        region_keys.sort();
        region_keys.dedup();
        let regions: RegionSet = Set::load(&in_dir.join("regions"), &region_keys, "json");

        let mut react_keys = if let Some(keys) = &self.reacts {
            keys.clone()
        } else {
            Vec::new()
        };
        react_keys.sort();
        react_keys.dedup();
        let reacts: ReactSet = Set::load(&in_dir.join("reactions"), &react_keys, "json");

        let mut light_keys = if let Some(keys) = &self.lights {
            keys.clone()
        } else {
            Vec::new()
        };
        light_keys.sort();
        light_keys.dedup();
        let lights: LightSet = Set::load(&in_dir.join("lights"), &light_keys, "json");

        let mut mat_keys = inters.mat_keys();
        mat_keys.sort();
        mat_keys.dedup();
        let mats: MatSet = Set::load(&in_dir.join("materials"), &mat_keys, "json");

        let mut state_keys = regions.state_keys();
        state_keys.sort();
        state_keys.dedup();
        let states: StateSet = Set::load(&in_dir.join("states"), &state_keys, "json");

        let mut spec_keys = reacts.spec_keys();
        spec_keys.append(&mut states.spec_keys());
        spec_keys.sort();
        spec_keys.dedup();
        let specs: SpecSet = Set::load(&in_dir.join("species"), &spec_keys, "json");

        let mut surf_keys = inters.surf_keys();
        surf_keys.append(&mut regions.surf_keys());
        surf_keys.append(&mut lights.surf_keys());
        surf_keys.sort();
        surf_keys.dedup();
        let proto_surfs: Set<SurfKey, SurfaceForm> =
            Set::load(&in_dir.join("surfaces"), &surf_keys, "json");

        let mut mesh_keys: Vec<_> = proto_surfs
            .map()
            .values()
            .map(|surf| surf.mesh().clone())
            .collect();
        mesh_keys.sort();
        mesh_keys.dedup();
        let meshes: MeshSet = Set::load(&in_dir.join("meshes"), &mesh_keys, "obj");

        let surfs = SurfSet::build(&proto_surfs, &meshes);

        WorldVerse::new(inters, regions, reacts, lights, mats, states, specs, surfs)
    }
}
