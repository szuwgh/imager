use std::io;
use std::path::PathBuf;
pub fn get_exe_path() -> Result<PathBuf, io::Error> {
    std::env::current_exe()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_exe_path() {
        let path = get_exe_path();
        println!("{:?}", path);
    }
}
