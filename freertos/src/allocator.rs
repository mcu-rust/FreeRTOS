use crate::base::*;
use crate::shim::*;
use core::alloc::{GlobalAlloc, Layout};

/**
Use with:

    #[global_allocator]
    static GLOBAL: FreeRtosAllocator = FreeRtosAllocator;
*/

pub struct FreeRtosAllocator;

#[cfg(feature = "heap_free_size")]
impl FreeRtosAllocator {
    pub fn get_free_size(&self) -> usize {
        unsafe { freertos_rs_get_free_heap_size() }
    }

    pub fn get_min_free_size(&self) -> usize {
        unsafe { freertos_rs_get_minimum_free_heap_size() }
    }

    pub fn reset_min_free_size(&self) {
        unsafe { freertos_rs_reset_minimum_free_heap_size() }
    }
}

unsafe impl GlobalAlloc for FreeRtosAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let res = unsafe { freertos_rs_pvPortMalloc(layout.size() as u32) };
        return res as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { freertos_rs_vPortFree(ptr as FreeRtosVoidPtr) }
    }
}
