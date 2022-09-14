pub mod repo;
pub mod package;
pub enum ZyypCallback {
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
    use crate::{package::package::install, ZyypCallback};

    #[test]
    fn it_works() {
        let packages:Vec<String> = vec!["java-13-openjdk".to_string()];
        assert!(matches!(install("sudo", packages), ZyypCallback::ZyppNotImplemented));
    }
}