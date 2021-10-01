use crate::prcpsp::rn_resource_usage::RnResourceUsage as RnResourceUsage;
use core::cmp::Ordering;

#[derive(Debug,Clone)]
pub struct Activity {
    pub id : u32,
    name : String,
    anteccessors : Vec<u32>,
    successors : Vec<u32>,
    supplies : Vec<RnResourceUsage>
}

impl Activity {
    pub fn new(id: u32, name: String, anteccessors: Vec<u32>, successors: Vec<u32>, supplies: Vec<RnResourceUsage> ) -> Activity {
        Activity{ id, name, anteccessors, successors, supplies }
    }

    pub fn add_anteccessor(&mut self, id_anteccessor: u32) {
        self.anteccessors.push(id_anteccessor);
    }

    pub fn add_successor(&mut self, id_successor: u32) {
        self.successors.push(id_successor);
    }
}

impl PartialEq for Activity {
    fn eq(&self, other : &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Activity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Eq for Activity {}

impl Ord for Activity {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.id == other.id {
            return Ordering::Equal;
        } else if self.id < other.id {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }
}
