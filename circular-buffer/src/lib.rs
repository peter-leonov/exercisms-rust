// one day when I'm mature enough to switch to the unstable Rust…
// extern crate alloc;
// use alloc::raw_vec::RawVec;

pub struct CircularBuffer<T> {
    capacity: usize,
    head: usize,
    back: usize,
    buffer: Vec<Option<T>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

const MAX_CAPACITY: usize = usize::MAX >> 2;

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity < MAX_CAPACITY);

        let mut buffer = Vec::with_capacity(capacity);

        // some day later…
        // unsafe {
        //     buffer.set_len(capacity);
        // }

        for _ in 0..capacity {
            buffer.push(None);
        }

        Self {
            capacity,
            head: 0,
            back: 0,
            buffer,
        }
    }

    pub fn is_full(&self) -> bool {
        self.head - self.back == self.capacity
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            Err(Error::FullBuffer)
        } else {
            self.buffer[self.head % self.capacity] = Some(element);
            self.head += 1;
            Ok(())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.back == self.head
    }

    #[inline(always)]
    pub fn wrap(&mut self) {
        if self.back > self.capacity {
            self.back -= self.capacity;
            self.head -= self.capacity;
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            Err(Error::EmptyBuffer)
        } else {
            let element = self
                .buffer
                .get_mut(self.back % self.capacity)
                .unwrap()
                .take()
                .unwrap();
            self.back += 1;
            self.wrap();
            Ok(element)
        }
    }

    pub fn clear(&mut self) {
        // we need to properly drop the remaining elements
        while let Ok(_) = self.read() {}
    }

    pub fn overwrite(&mut self, element: T) {
        if self.is_full() {
            self.read().unwrap();
        }
        self.write(element).unwrap();
    }
}
