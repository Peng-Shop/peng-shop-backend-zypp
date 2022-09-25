#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("zypp/ZYppFactory.h");

        type ZyppFactory;
        type ZYpp;

        fn instance() -> UniquePtr<ZyppFactory>;
        fn getZYpp() -> UniquePtr<ZYpp>;
    }
}

fn main(){
    let client = ffi::instance();
}