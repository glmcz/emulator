struct Stack <T>{
    inner: Vec<T>,
    top_stack: usize
}

impl<T> Stack <T> {
    pub fn init() -> Self {
        Stack {
            inner: Vec::new(),
            top_stack: 0
        }
    }
    // FIFO first in first out
    // LIFO last in first out
    pub fn push(&mut self, element: T) {
        self.top_stack += 1;
        self.inner.insert(0, element); //push element always into the top
        // LIFO
        // self.inner.insert(self.top_stack, element);
        // self.top_stack += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.inner.is_empty() {
            return None;
        }
        let element = self.inner.remove(0);
        self.top_stack -= 1;
        Some(element)
    }
    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.top_stack == 0
    }
}

// Better performance thant Vec stack, because insert is in case of LIFO
// can involve mem reallocation at the end of Vec.
// It happend in the case when we overgrov defined Vec capacity, so
// reallocation has to be performend
use std::collections::VecDeque;

struct FifoStack<T> {
    data: VecDeque<T>,
}

impl<T> FifoStack<T> {
    fn new() -> Self {
        FifoStack { data: VecDeque::new() }
    }

    // Push element to the front of the queue (FIFO behavior)
    fn push(&mut self, element: T) {
        self.data.push_front(element);
    }

    // Pop element from the front of the queue (FIFO behavior)
    fn pop(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
fn main() {

    let mut stack: Stack<i32> = Stack::init();
    stack.push(5);
    stack.push(8);
    let el = stack.pop();
    assert_eq!(el.unwrap(), 8, "Successfully return removed element");

}