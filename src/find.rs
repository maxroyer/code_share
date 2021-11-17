#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct FindTools {
    pub query_buf: String,
    match_locations: Vec<usize>,
}

impl Default for FindTools {
    fn default() -> Self {
        FindTools { query_buf: String::new(), match_locations: Vec::new() }
    }
}

impl FindTools {
    pub fn add_match(&mut self, loc: usize) {
        self.match_locations.push(loc);
    }
    pub fn reset(&mut self) { *self = FindTools::default(); }
    pub fn get_query(&self) -> String { self.query_buf.clone() }
}