#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct AppConfig {
    font_size: u32,
    pub line_nums: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig { font_size: 15, line_nums: true, }
    }
}

impl AppConfig {
    pub fn _set_font_size(&mut self, size: u32) { self.font_size = size; }
    pub fn get_font_size(&self) -> f32 { self.font_size as f32 }
    pub fn dec_font_size(&mut self) {
        if self.font_size > 0 {
            self.font_size -= 1;
        }
    }
    pub fn inc_font_size(&mut self) {
        if self.font_size > 0 {
            self.font_size += 1;
        }
    }

    pub fn _set_line_nums(&mut self, state: bool) {
        self.line_nums = state;
    }
}
