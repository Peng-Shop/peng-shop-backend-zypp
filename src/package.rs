use std::process::{Command, Stdio};

use crate::ZyypCallback;

const ZYPP:&str="zypper";

pub fn install(sudo_tool:&str, packages:Vec<String>) -> ZyypCallback {
    let pkgs:String = packages.concat();

    let process = Command::new(sudo_tool)
    .arg(ZYPP)
    .arg("in")
    .arg("-y")
    .arg(pkgs.as_str())
    .stdout(Stdio::piped())
    .output()
    .expect("Failed to install packages");

    let result = String::from_utf8_lossy(&process.stdout).to_string();
    let v: Vec<&str> = result.split("\n").collect();
    dbg!(v);

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