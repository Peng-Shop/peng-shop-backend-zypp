pub mod repo;
pub mod package;
pub enum ZyypCallback {
    ZyppPackagesNotFound(Vec<String>),
    ZyppSuccess,
    ZyppTimeOut,
    ZyppRepoExists,
    ZyppNotImplemented
}

#[cfg(test)]
mod tests {
    use crate::{package::{self, install}, ZyypCallback};


    #[test]
    fn it_works() {
        let packages:Vec<String> = vec!["java-13-openjdk".to_string()];
        assert!(matches!(install("sudo", packages), ZyypCallback::ZyppNotImplemented));
    }
}