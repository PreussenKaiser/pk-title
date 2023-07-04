pub struct Size {
    w: u32,
    h: u32
}

impl Size {
    pub fn new(w: u32, h: u32) -> Self {
        Self { w, h }
    }

    pub fn w(&self) -> u32 { self.w }
    pub fn h(&self) -> u32 { self.h }
}