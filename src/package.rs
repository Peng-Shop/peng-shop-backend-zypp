//! Bindings for libzypp's packagemanager (Package.cc)
pub mod package {
    use std::process::{Command, Stdio};
    use crate::ZyppCallback;

    const ZYPP:&str="zypper";
    
    #[link(name = "zypp")]
    extern {
        fn IsAvailable(tag:&str) -> bool ;
    }
    
    pub fn install(sudo_tool:&str, packages:Vec<String>) -> ZyppCallback {    
        return ZyppCallback::ZyppNotImplemented;
    }
    
pub fn uninstall(sudo_tool:&str, clean_deps:bool) -> ZyppCallback {
    return ZyppCallback::ZyppNotImplemented;
    }
    
    pub fn info(package:&str) -> String{
        return "NotImplemented".to_string();
    }
    
pub fn add_lock(sudo_tool:&str, packages:Vec<String>) -> ZyppCallback{
    return ZyppCallback::ZyppNotImplemented;
    }
    
    pub fn remove_lock(sudo_tool:&str, packages:Vec<String>) -> ZyppCallback {
        return ZyppCallback::ZyppNotImplemented;
    }
    
    /// Test if the given tag is available
    ///  
    /// A tag is consideret to be:
    /// - a packagename
    ///     - Also quereis provides and required packages acroding to libzypp
    /// - filename
    pub fn is_available(tag:&str){}
}