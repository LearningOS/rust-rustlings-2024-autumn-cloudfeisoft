struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.
    unsafe {
        // We transmute the raw pointer back into a Box<Foo>, taking ownership.
        // This is safe because the contract ensures that the pointer is valid
        // and points to an allocated `Foo`.
        Box::from_raw(ptr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let mut data = Box::new(Foo { a: 1, b: None });
        data.b = Some("hello".to_owned()); // Initialize `b` with "hello"

        let ptr_1 = &*data as *const Foo as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &*ret as *const Foo as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
