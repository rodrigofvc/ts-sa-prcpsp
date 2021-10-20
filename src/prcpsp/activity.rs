use crate::prcpsp::rn_resource_usage::RnResourceUsage as RnResourceUsage;
use core::cmp::Ordering;

#[derive(Debug,Clone)]
pub struct Activity {
    pub id : u32,
    pub parent: i32,
    pub name : String,
    pub predecessors : Vec<u32>,
    pub successors : Vec<u32>,
    pub supplies : Vec<RnResourceUsage>,
    pub duration: u32,
    pub start_time : i32,
}

impl Activity {
    pub fn new(id: u32, parent: i32, name: String, predecessors: Vec<u32>, successors: Vec<u32>, supplies: Vec<RnResourceUsage>, duration: u32, start_time: i32 ) -> Activity {
        Activity{ id, parent, name, predecessors, successors, supplies, duration, start_time }
    }

    pub fn add_anteccessor(&mut self, id_anteccessor: u32) {
        self.predecessors.push(id_anteccessor);
    }

    pub fn add_successor(&mut self, id_successor: u32) {
        self.successors.push(id_successor);
    }

    pub fn is_predecessor(&self, activity: Activity) -> bool {
        let is_predecessor = self.predecessors.iter().find(|&&x|x == activity.id);
        match is_predecessor {
            Some(_) => return true,
            _ => return false
        }
    }

    pub fn is_successor (&self, activity: Activity) -> bool {
        let is_successor = self.successors.iter().find(|&&x|x == activity.id);
        match is_successor {
            Some(_) => return true,
            _ => return false
        }
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
