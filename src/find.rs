#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct FindTools {
    pub query_buf: String,
    pub match_locations: Vec<usize>,
    selected_loc: usize,
    pub initial_click_made: bool,
    pub replace_mode: bool,
    pub replace_buf: String,
}

impl Default for FindTools {
    fn default() -> Self {
        FindTools {
            query_buf: String::new(),
            match_locations: Vec::new(),
            selected_loc: 0,
            initial_click_made: false,
            replace_mode: false,
            replace_buf: String::new(),
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

    pub fn update_matches(&mut self) {
        //call after using replace on a match4
        let find_len = self.query_buf.chars().count();
        let rep_len = self.replace_buf.chars().count();
        if rep_len > find_len {
            for i in self.selected_loc..self.match_locations.len() {
                self.match_locations[i] += rep_len - find_len;
            }
        } else if rep_len < find_len {
            for i in self.selected_loc+1..self.match_locations.len() {
                self.match_locations[i] -= find_len - rep_len;
            }
        }
        //selected location shoud stay the same index, unless it should wrap
        //around to the start
        match self.selected_loc == self.match_locations.len() - 1 {
            true => {
                self.match_locations.remove(self.selected_loc);
                self.selected_loc = 0
            },
            false => {
                self.match_locations.remove(self.selected_loc);
            ()
            }
        }
    }

    pub fn get_query(&self) -> String {
        self.query_buf.clone()
    }

    pub fn get_current_match(&mut self) -> Option<(usize, usize)> {
        //Returns Some(current match starting index, current query len)
        if !self.match_locations.is_empty() {
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

