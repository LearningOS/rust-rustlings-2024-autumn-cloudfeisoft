impl<T: Ord> LinkedList<T> {
    pub fn merge(mut list_a: Self, mut list_b: Self) -> Self {
        let mut result = Self::new();

        // Use raw pointers to avoid repeated unsafe blocks
        let (mut ptr_a, mut ptr_b) = (list_a.start, list_b.start);

        // While both lists have elements
        while let (Some(node_a), Some(node_b)) = (ptr_a.map(|n| unsafe { &*n.as_ptr() }), ptr_b.map(|n| unsafe { &*n.as_ptr() })) {
            if node_a.val < node_b.val {
                // Append node from list_a to the result
                result.add(node_a.val);
                ptr_a = node_a.next;
            } else {
                // Append node from list_b to the result
                result.add(node_b.val);
                ptr_b = node_b.next;
            }
        }

        // Append remaining elements from list_a
        if let Some(mut remaining_a) = ptr_a {
            while let Some(node) = remaining_a {
                result.add(unsafe { (*node.as_ptr()).val });
                remaining_a = unsafe { (*node.as_ptr()).next };
            }
        }

        // Append remaining elements from list_b
        if let Some(mut remaining_b) = ptr_b {
            while let Some(node) = remaining_b {
                result.add(unsafe { (*node.as_ptr()).val });
                remaining_b = unsafe { (*node.as_ptr()).next };
            }
        }

        result
    }
}