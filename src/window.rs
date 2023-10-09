use crate::console::CharInfo;

/// rectangular region of the console
#[derive(Debug, Clone)]
pub struct Window {
    #[doc(hidden)]
    pub data: [i32; 4],
    #[doc(hidden)]
    pub buffer: Vec<CharInfo>,
    #[doc(hidden)]
    cached_buffer_count: usize,
    #[doc(hidden)]
    cached_buffer_size: (i32, i32),
}

impl Window {
    /// create an instance of the Window structure, allocating w * h [CharInfo] to store
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            data: [x, y, w, h],
            buffer: vec![CharInfo::default(); (w * h) as usize],
            cached_buffer_count: (w * h) as usize,
            cached_buffer_size: (w, h),
        }
    }

    /// gets the position of the window top-left corner
    pub fn position(&self) -> (i32, i32) {
        (self.data[0], self.data[1])
    }

    /// gets the size of the buffer as tuple of width, height
    pub fn buffer_size(&self) -> (i32, i32) {
        self.cached_buffer_size
    }

    /// gets the total size of the buffer as number of cells
    pub fn buffer_count(&self) -> usize {
        self.cached_buffer_count
    }
}
