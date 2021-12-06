#[derive(Clone,Debug)]
pub struct TabuMv {
    pub movements: Vec<u32>,
    pub tabu_time: u32
}

impl TabuMv {
    pub fn new(movements: Vec<u32>, tabu_time: u32) -> TabuMv {
        TabuMv { movements, tabu_time }
    }
    pub fn is_tabu(&self, activities: Vec<u32>) -> bool {
        if activities.len() != self.movements.len() {
            return false;
        }
        for a in activities {
            if !self.movements.contains(&a) {
                return false;
            }
        }
        return true;
    }
}
