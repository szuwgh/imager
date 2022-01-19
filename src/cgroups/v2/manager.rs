use std::path::PathBuf;

use crate::cgroups::SMOG;
struct CgroupsManager {
    root_path: PathBuf,
    cgroups_path: PathBuf,
}

impl CgroupsManager {
    pub fn new(root_path: PathBuf, container_id: &str) -> CgroupsManager {
        let cgroups_path = PathBuf::from(format!("/{}/{}", SMOG, container_id));
        Self {
            root_path: root_path,
            cgroups_path: cgroups_path,
        }
    }

    fn create_unified_cgroup() -> Result<()> {}

    fn add_task() {}
}
