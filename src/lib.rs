pub enum ZyypCallback {
    ZyppPackagesNotFound(Vec<String>),
    ZyppSuccess,
    ZyppTimeOut,
    ZyppRepoExists,
    ZyppNotImplemented
}

pub fn install() -> ZyypCallback {
    return ZyypCallback::ZyppNotImplemented;
}

pub fn uninstall(clean_deps:bool) -> ZyypCallback {
    return ZyypCallback::ZyppNotImplemented;
}

pub fn refresh(force_refresh:bool) -> ZyypCallback {
    return ZyypCallback::ZyppNotImplemented;
}

pub fn info(package:&str) -> String{
    return "NotImplemented".to_string();
}

pub fn add_lock(packages:Vec<String>) -> ZyypCallback{
    return ZyypCallback::ZyppNotImplemented;
}

pub fn remove_lock(packages:Vec<String>) -> ZyypCallback {
    return ZyypCallback::ZyppNotImplemented;
}

pub fn add_repo(url:&str, name:&str, priority:i16) -> ZyypCallback {
    return ZyypCallback::ZyppNotImplemented;
}

pub fn remove_repo(name:&str) -> ZyypCallback {
    return ZyypCallback::ZyppNotImplemented;
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        
    }
}
