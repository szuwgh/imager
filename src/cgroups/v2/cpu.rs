use super::subsystem::SubSystem;
use crate::cgroups::common::{self, ControllerOpt};
use crate::oci::oci::LinuxCpu;
use anyhow::Result;
use std::path::Path;

const CGROUP_CPU_WEIGHT: &str = "cpu.weight";
const CGROUP_CPU_MAX: &str = "cpu.max";
const DEFAULT_PERIOD: &str = "100000";
const UNRESTRICTED_QUOTA: &str = "max";

pub struct Cpu {}

impl SubSystem for Cpu {
    fn apply(controller_opt: &ControllerOpt, cgroup_path: &Path) -> Result<()> {
        if let Some(cpu) = &controller_opt.resources.cpu {
            Self::apply(&cpu, cgroup_path)?;
        }
        Ok(())
    }
}

impl Cpu {
    fn apply(cpu: &LinuxCpu, path: &Path) -> Result<()> {
        if let Some(mut shares) = cpu.shares {
            shares = Self::convert_shares_to_cgroup2(shares);
            if shares != 0 {
                common::write_cgroup_file(path.join(CGROUP_CPU_WEIGHT), shares)?;
            }
        }
        let mut quota_string = UNRESTRICTED_QUOTA.to_owned();
        if let Some(quota) = cpu.quota {
            if quota > 0 {
                quota_string = quota.to_string();
            }
        }

        let mut period_string: String = DEFAULT_PERIOD.to_owned();
        if let Some(period) = cpu.period {
            if period > 0 {
                period_string = period.to_string();
            }
        }

        let max = quota_string + " " + &period_string;
        common::write_cgroup_file_str(path.join(CGROUP_CPU_MAX), &max)?;
        Ok(())
    }

    fn convert_shares_to_cgroup2(shares: u64) -> u64 {
        if shares == 0 {
            return 0;
        }
        1 + ((shares - 2) * 9999) / 262142
    }
}
