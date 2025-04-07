#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0)) // Remove from the front
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

pub struct myStack<T> {
    q1: Queue<T>, // Primary queue
    q2: Queue<T>, // Helper queue for pop operations
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    // Push an element onto the stack
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem); // Just enqueue into q1
    }

    // Pop an element from the stack
    pub fn pop(&mut self) -> Result<T, &str> {
        // If q1 is empty, the stack is empty
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        // Transfer all elements except the last from q1 to q2
        while self.q1.size() > 1 {
            if let Ok(elem) = self.q1.dequeue() {
                self.q2.enqueue(elem);
            }
        }

        // The last element left in q1 is the top of the stack
        if let Ok(top) = self.q1.dequeue() {
            // Swap the queues: q1 becomes the new empty queue
            std::mem::swap(&mut self.q1, &mut self.q2);
            return Ok(top); // Return the popped element
        }

        Err("Stack is empty") // In case of failure, although this shouldn't happen
    }

    // Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3)); // Should pop 3
        assert_eq!(s.pop(), Ok(2)); // Should pop 2
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false); // Stack is not empty
        assert_eq!(s.pop(), Ok(5)); // Should pop 5
        assert_eq!(s.pop(), Ok(4)); // Should pop 4
        assert_eq!(s.pop(), Ok(1)); // Should pop 1
        assert_eq!(s.pop(), Err("Stack is empty")); // Stack is empty
        assert_eq!(s.is_empty(), true); // Stack is empty
    }
}
