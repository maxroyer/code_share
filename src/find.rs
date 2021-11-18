#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct FindTools {
    pub query_buf: String,
    match_locations: Vec<usize>,
    selected_loc: usize,
    pub initial_click_made: bool,
}

impl Default for FindTools {
    fn default() -> Self {
        FindTools {
            query_buf: String::new(),
            match_locations: Vec::new(),
            selected_loc: 0,
            initial_click_made: false,
        }
    }
}

impl FindTools {
    pub fn add_match(&mut self, loc: usize) {
        self.match_locations.push(loc);
    }
    pub fn full_reset(&mut self) {
        *self = FindTools::default();
    }

    pub fn reset_matches(&mut self) {
        self.match_locations = Vec::new();
        self.selected_loc = 0;
        self.initial_click_made = false;
    }

    pub fn get_query(&self) -> String {
        self.query_buf.clone()
    }

    pub fn get_current_match(&mut self) -> Option<(usize, usize)> {
        //Returns Some(current match starting index, current query len)
        if self.match_locations.len() != 0 {
            self.initial_click_made = true;
            Some((
                self.match_locations[self.selected_loc],
                self.query_buf.chars().count(),
            ))
        } else {
            None
        }
    }

    pub fn number_of_matches(&self) -> usize {
        self.match_locations.len()
    }

    pub fn selected_loc_inc(&mut self) {
        if self.selected_loc == self.match_locations.len() - 1 {
            self.selected_loc = 0;
        } else {
            self.selected_loc += 1;
        }
    }
    pub fn selected_loc_dec(&mut self) {
        if self.selected_loc == 0 {
            self.selected_loc = self.match_locations.len() - 1;
        } else {
            self.selected_loc -= 1;
        }
    }
}
