use instant::Instant;

pub struct FrameTimer {
    pub frame_count: u64,
    pub frames: u64,
    pub epoch: Instant,
}

impl Default for FrameTimer {
    fn default() -> Self {
        Self {
            frames: 0,
            frame_count: 0,
            epoch: Instant::now(),
        }
    }
}

impl FrameTimer {
    pub fn frame_count(&mut self, region: &crate::region::Region) -> u64 {
        let diff = Instant::now().duration_since(self.epoch);
        let rate = if region.is_pal() {
            0.049701459
        } else {
            0.05992274
        };
        let frames = (diff.as_millis() as f64 * rate) as u64;
        // self.emu.gfx.framerate()
        self.frame_count = frames - self.frames;
        self.frames = frames;
        self.frame_count
    }
    /// use when unpausing
    pub fn reset_epoch(&mut self) {
        self.epoch = Instant::now();
    }
}
