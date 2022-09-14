pub mod repo {
    #[link(name = "zypp")]
    extern {
        
    }

    use crate::ZyppCallback;
    
    pub fn refresh(sudo_tool:&str, force_refresh:bool) -> ZyppCallback {
        return ZyppCallback::ZyppNotImplemented;
    }
    
    pub fn add_repo(sudo_tool:&str, url:&str, name:&str, priority:i32) -> ZyppCallback {
        let x = unsafe {
            
        };
        return ZyppCallback::ZyppNotImplemented;
    }
    
    pub fn remove_repo(sudo_tool:&str, name:&str) -> ZyppCallback {
        return ZyppCallback::ZyppNotImplemented;
    }
}

#[cfg(test)]
mod tests {
    use crate::ZyppCallback;
    use super::repo::add_repo;
    
    #[test]
    fn it_works() {
        assert!(matches!(add_repo("sudo", "url", "name", 90), ZyppCallback::ZyppNotImplemented));
    }
}