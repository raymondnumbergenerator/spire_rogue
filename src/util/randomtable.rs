use rltk::RandomNumberGenerator;

#[derive(Default)]
pub struct RandomTable<T: Copy> {
    entries: Vec<T>,
    weights: Vec<i32>,
    total_weight: i32,
}

impl<T: Copy> RandomTable<T> {
    pub fn new() -> RandomTable<T> {
        RandomTable{ entries: Vec::new(), weights: Vec::new(), total_weight: 0 }
    }
    
    pub fn add(mut self, entry: T, weight: i32) -> RandomTable<T> {
        self.total_weight += weight;
        self.entries.push(entry);
        self.weights.push(weight);
        self
    }

    pub fn roll(&self, rng: &mut RandomNumberGenerator) -> Option<T> {
        if self.total_weight == 0 { return None; }
        let mut roll = rng.roll_dice(1, self.total_weight) - 1;
        let mut index: usize = 0;

        while roll >= 0 {
            if roll < self.weights[index] {
                return Some(self.entries[index].clone());
            }

            roll -= self.weights[index];
            index += 1;
        }

        None
    }
}