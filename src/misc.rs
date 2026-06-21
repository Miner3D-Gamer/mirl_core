#[must_use]
/// Turn a hex string into a u32 value
pub fn hex_to_number(hex: &str) -> Option<u32> {
    u32::from_str_radix(hex, 16).ok()
}
#[must_use]
/// Turn a u32 into a hex string
pub fn color_to_hex(num: u32) -> String {
    format!("{num:x}")
}

/// A temporary function until either of the following:
/// - `#![feature(vec_try_remove)]` becomes stable
///
/// - Miri starts supporting `#![feature(vec_try_remove)]`
pub fn vec_try_remove<T>(list: &mut Vec<T>, index: usize) -> Option<T> {
    let len = list.len();
    if index >= len {
        return None;
    }
    unsafe {
        // infallible
        let ret;
        {
            // the place we are taking from.
            let ptr = list.as_mut_ptr().add(index);
            // copy it out, unsafely having a copy of the value on
            // the stack and in the vector at the same time.
            ret = core::ptr::read(ptr);

            // Shift everything down to fill in that spot.
            core::ptr::copy(ptr.add(1), ptr, len - index - 1);
        }
        list.set_len(len - 1);
        Some(ret)
    }
}
/// Remove an item from a vec without shifting all values or retaining order
///
/// # Safety
/// The caller must ensure that `index` is strictly less than `vec.len()`
pub unsafe fn vec_unchecked_swap_remove<T>(vec: &mut Vec<T>, index: usize) -> T {
    let len = vec.len();

    // 1. Read the item out of the vector (takes ownership)
    let base_ptr = vec.as_mut_ptr();
    unsafe {
        let removed_item = std::ptr::read(base_ptr.add(index));

        // 2. If it's not the last element, move the last element to the cleared slot
        if index < len - 1 {
            std::ptr::copy_nonoverlapping(base_ptr.add(len - 1), base_ptr.add(index), 1);
        }

        vec.set_len(len - 1);

        removed_item
    }
}
