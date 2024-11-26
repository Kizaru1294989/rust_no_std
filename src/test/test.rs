#[cfg(test)]
mod tests {
    use core::alloc::Layout;
    use crate::allocator::SimpleAllocator;

    #[test]
    fn test_allocation() {
        let layout = Layout::from_size_align(16, 8).unwrap();
        unsafe {
            let ptr = crate::allocator::GLOBAL_ALLOCATOR.alloc(layout);
            assert!(!ptr.is_null(), "Allocation échouée");
        }
    }
}
