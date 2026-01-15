use crate::utils::*;

#[cfg(feature = "assert-hook")]
mod assert_hook {
    use core::cell::OnceCell;

    type Callback = fn();

    pub struct FreeRtosHooks {
        on_assert: OnceCell<Callback>,
    }

    impl FreeRtosHooks {
        pub fn set_on_assert(&self, c: Callback) -> Result<(), Callback> {
            self.on_assert.set(c)
        }

        pub(super) fn do_on_assert(&self) {
            if let Some(cb) = self.on_assert.get() {
                cb()
            }
        }
    }

    // SAFETY: must only be set before the scheduler starts and accessed after the
    // kernel has asserted, both being single threaded situations.
    unsafe impl Sync for FreeRtosHooks {}

    pub static FREERTOS_HOOKS: FreeRtosHooks = FreeRtosHooks {
        on_assert: OnceCell::new(),
    };
}
#[cfg(feature = "assert-hook")]
pub use assert_hook::*;

#[allow(unused_doc_comments)]
#[unsafe(no_mangle)]
pub extern "C" fn assert_callback(line: u32, file_name_ptr: *const u8) {
    let file_name = unsafe { str_from_c_string(file_name_ptr).unwrap_or("Unknown") };

    #[cfg(feature = "assert-hook")]
    FREERTOS_HOOKS.do_on_assert();

    // we can't print without std yet.
    // TODO: make the macro work for debug UART? Or use Panic here?
    // println!("ASSERT: {} {}", line, file_name);
    panic!("FreeRTOS ASSERT: {}:{}", file_name, line);
    //loop {}
}
