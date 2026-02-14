pub struct CircularBuffer<T> {
    data: Vec<Option<T>>,
    head: usize,
    tail: usize,
    capacity: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![None; capacity],
            head: 0,
            tail: 0,
            capacity,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.data[self.head].is_some() {
            Err(Error::FullBuffer)
        } else {
            self.data[self.head] = Some(element);
            self.head = (self.head + 1) % self.capacity;
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.data[self.tail].is_none() {
            Err(Error::EmptyBuffer)
        } else {
            let ret = Ok(self.data[self.tail].take().unwrap());
            self.tail = (self.tail + 1) % self.capacity;
            ret
        }
    }

    pub fn clear(&mut self) {
        for slot in &mut self.data[self.tail..self.head + 1] {
            slot.take();
        }
        self.head = 0;
        self.tail = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.data[self.head].is_none() {
            self.write(element).unwrap();
        } else {
            _ = self.read();
            self.write(element).unwrap();
        }
    }
}
