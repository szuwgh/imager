use procfs::process::Process;

pub struct Manager {}

impl Manager {}

pub fn get_subsystem_mount_point() {
    println!("{:?}", Process::myself().unwrap().mountinfo().unwrap());
}
