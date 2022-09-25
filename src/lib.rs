pub mod repo;
pub mod package;
pub mod ffi;

pub enum ZyppCallback {
    ZyppPackagesNotFound(Vec<String>),
    ZyppSuccess,
    ZyppTimeOut,
    ZyppRepoExists,
    ZyppNotImplemented
}

pub enum ZyppResKind {
    NoKind,
    Package,
    Patch,
    Pattern,
    Product,
    SrcPackage,
    Application
}

impl ZyppResKind {
    fn value(&self) -> &str {
        match *self {
            ZyppResKind::Application => "application",
            ZyppResKind::NoKind => "",
            ZyppResKind::Package => "package",
            ZyppResKind::Patch => "patch",
            ZyppResKind::Pattern => "pattern",
            ZyppResKind::Product => "product",
            ZyppResKind::SrcPackage => "srcpackage"
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{package::package::install, ZyppCallback};

    #[test]
    fn it_works() {
        let packages:Vec<String> = vec!["java-13-openjdk".to_string()];
        assert!(matches!(install("sudo", packages), ZyppCallback::ZyppNotImplemented));
    }
}