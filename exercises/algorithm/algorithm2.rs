impl<T> LinkedList<T> {
    pub fn reverse(&mut self) {
        let mut current = self.start;
        let mut prev = None;
        while let Some(node) = current {
            let next = unsafe { (*node.as_ptr()).next };
            unsafe { (*node.as_ptr()).next = prev; }
            unsafe { (*node.as_ptr()).prev = next; }
            prev = Some(node);
            current = next;
        }
        self.start = prev;
        self.end = None;
    }
}