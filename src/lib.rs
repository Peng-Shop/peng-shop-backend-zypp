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

    #[test]
    fn it_works() {
        
    }
}