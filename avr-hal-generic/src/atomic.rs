//! Atomic helpers
//!
//! Intrinsic functions to do atomic compare and swap operations.
//! Automatically called by the compiler when necessary.

#[allow(unused_imports)]
use paste::paste;

#[allow(unused_macros)]
macro_rules! sync_val_compare_and_swap_n {
    ( $n:expr, $t:expr ) => (
        paste! {
            #[no_mangle]
            pub extern "C" fn [<__sync_val_compare_and_swap_ $n>](ptr: &mut $t, old: $t, new: $t) -> $t {
                avr_device::interrupt::free(|_critical_section| {
                    let ptr_val = *ptr;
                    if ptr_val == old {
                        *ptr = new;
                    }
                    ptr_val
                })
            }
        }
    )
}

#[cfg(target_arch = "avr")]
sync_val_compare_and_swap_n!(1, u8);
#[cfg(target_arch = "avr")]
sync_val_compare_and_swap_n!(2, u16);
#[cfg(target_arch = "avr")]
sync_val_compare_and_swap_n!(4, u32);
