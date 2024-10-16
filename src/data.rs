use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct DataSheet {
    pub size_bytes:usize,
    pub max_size_bytes:usize,
    pub data: [u8;1024],
}

impl DataSheet {
    pub fn new() -> Self {
        Self{
            size_bytes: 0,
            max_size_bytes:1024,
            data: [0;1024]
        }
    }

    pub fn random() -> Self {
        let mut data = [0u8;1024];
        rand::thread_rng().fill(&mut data[..]);
        Self{
            size_bytes: 0,
            max_size_bytes:1024,
            data: data,
        }

    }
}
