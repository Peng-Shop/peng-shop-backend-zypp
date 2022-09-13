use crate::ZyypCallback;

pub fn install(sudo_tool:&str) -> ZyypCallback {
    return ZyypCallback::ZyppNotImplemented;
}

pub fn uninstall(sudo_tool:&str, clean_deps:bool) -> ZyypCallback {
    return ZyypCallback::ZyppNotImplemented;
}

pub fn info(package:&str) -> String{
    return "NotImplemented".to_string();
}

pub fn add_lock(sudo_tool:&str, packages:Vec<String>) -> ZyypCallback{
    return ZyypCallback::ZyppNotImplemented;
}

pub fn remove_lock(sudo_tool:&str, packages:Vec<String>) -> ZyypCallback {
    return ZyypCallback::ZyppNotImplemented;
}