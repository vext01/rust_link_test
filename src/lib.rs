#![feature(link_args)]

//#[link_args="-lcstuff"]
extern "C" {
    fn c_func_in_lib() -> u64;
}

pub fn call_c() -> u64 {
    unsafe { c_func_in_lib() }
}

mod tests {
    use ::call_c;

    #[test]
    fn test() {
        assert_eq!(call_c(), 42);
    }
}
