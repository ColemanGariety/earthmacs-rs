use buffer::Buffer;

pub struct Window {
    pub buffer_index: usize,
}

impl Window {
    pub fn new(buffer_index: usize) -> Window {
        Window { buffer_index: buffer_index }
    }
}
