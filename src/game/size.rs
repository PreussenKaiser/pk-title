pub struct Size {
    w: u32,
    h: u32
}

impl Size {
    pub fn new(w: u32, h: u32) -> Self {
        return Self { w, h };
    }

    pub fn w(&self) -> u32 { return self.w }
    pub fn h(&self) -> u32 { return self.h }
}