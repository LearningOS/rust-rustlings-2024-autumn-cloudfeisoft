use std::mem;

/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // Safety notice for the code block
    ///
    /// Casting the address back to a pointer and dereferencing it to modify the value.
    /// This is safe because the address is guaranteed to be valid and points to a mutable `u32`.
    unsafe {
        let value_ptr = address as *mut u32;
        // Dereferencing the pointer to get a mutable reference to the value
        // and then modifying it.
        *value_ptr = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}