pub struct GameLog {
    pub entries: Vec<String>
}

impl GameLog {
    pub fn push(&mut self, msg: String) {
        self.entries.push(msg);
    }
}