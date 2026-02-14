use std::{
    borrow::Borrow,
    io::{Read, Write},
    iter::Cycle,
    slice::Iter,
};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        Self {
            key: key.as_ref().iter().cycle(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for byte in data {
            *byte ^= self.key.next().unwrap();
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'b, Data, Byte>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + 'b
    where
        Data: IntoIterator<Item = Byte> + 'b,
        Byte: Borrow<u8>,
    {
        data.into_iter()
            .map(|byte| self.key.next().unwrap() ^ byte.borrow())
    }

    pub fn reader(self, input: impl Read) -> impl Read {
        Reader {
            key: self.key,
            input,
        }
    }

    pub fn writer(self, destination: impl Write) -> impl Write {
        Writer {
            key: self.key,
            destination,
        }
    }
}

struct Reader<'a, Bytes: Read> {
    key: Cycle<Iter<'a, u8>>,
    input: Bytes,
}

impl<Bytes: Read> Read for Reader<'_, Bytes> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.input.read(buf).inspect(|_| {
            for byte in buf {
                *byte ^= self.key.next().unwrap();
            }
        })
    }
}

struct Writer<'a, Bytes: Write> {
    key: Cycle<Iter<'a, u8>>,
    destination: Bytes,
}

impl<Bytes: Write> Write for Writer<'_, Bytes> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.destination.write(&Vec::from_iter(
            buf.iter().map(|b| b ^ self.key.next().unwrap()),
        ))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.destination.flush()
    }
}
