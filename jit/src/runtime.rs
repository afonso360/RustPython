#[repr(C)]
struct JitRuntime {}

pub extern "C" fn jit_rt_get_global(vm: i64, num: u32) -> *const () {
    println!("Hello!!!!!!!! {}, num: {}", vm, num);
    std::ptr::null()
}
