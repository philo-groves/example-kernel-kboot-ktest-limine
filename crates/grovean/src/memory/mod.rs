#[cfg(any(test, target_arch = "x86_64"))]
pub mod frame_allocator;
pub mod memory_map;

pub fn init() {
    memory_map::init();
}
