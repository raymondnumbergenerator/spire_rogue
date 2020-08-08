pub struct Gamelog {
    pub entries: Vec<String>
}

impl Gamelog {
    pub fn push(&mut self, msg: String) {
        self.entries.push(msg);
    }
}