#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct AppConfig {
    font_size: u32,
    pub line_nums: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            font_size: 15,
            line_nums: true,
        }
    }
}

impl AppConfig {
    pub fn _set_font_size(&mut self, size: u32) {
        self.font_size = size;
    }
    pub fn get_font_size(&self) -> f32 {
        self.font_size as f32
    }
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

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct LineNumbers {
    num_string: String,
    pub line_count: usize,
}

impl Default for LineNumbers {
    fn default() -> Self {
        LineNumbers {
            num_string: String::from("~"),
            line_count: 0,
        }
    }
}

impl LineNumbers {
    pub fn generate(&mut self, current_count: usize) -> String {
        let Self {
            num_string,
            line_count,
        } = self;
        if *line_count != current_count {
            num_string.clear();
            let num_digits = LineNumbers::get_num_digits(current_count);

            for i in 1..=current_count {
                let leading_spaces = num_digits - LineNumbers::get_num_digits(i);
                let temp_str = format!("{:width$}{}", "", i, width = leading_spaces + 1);
                num_string.push_str(&format!("{}\n", temp_str));
            }

            num_string.push_str(&format!(" {:width$}~", "", width = num_digits - 1));
            return num_string.clone();
        }
        num_string.clone()
    }
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    fn get_num_digits(num: usize) -> usize {
        let mut num = num;
        let mut dig_count: usize = 1;
        while num / 10 > 0 {
            num /= 10;
            dig_count += 1;
        }
        dig_count
    }
}
