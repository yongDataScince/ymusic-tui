// --------- Objects types BEGIN ---------
pub struct Artist {
    pub id: u16,
}

pub struct Track {
    pub id: u64,
    pub title: String,
    pub artist: Artist,
    pub duration: u16
}


// --------- Objects types END ---------

// --------- URI types BEGIN ---------
pub enum SearchType {
  Tracks,
  Albums,
  Artists
}

impl ToString for SearchType {
    fn to_string(&self) -> String {
        use SearchType::*;
        match self {
            Tracks => String::from("tracks"),
            Albums => String::from("albums"),
            Artists => String::from("artists"),
        }
    }
}
// --------- URI types END ---------
