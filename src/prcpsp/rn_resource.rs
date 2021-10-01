use core::cmp::Ordering;

/* Represents a renewable resource */
#[derive(Debug,Clone)]
pub struct RnResource {
    pub id: u32,
    pub name: String,
    pub capacity: u32,
}

impl RnResource {
    pub fn new(id: u32, name: String, capacity: u32) -> RnResource {
        RnResource { id, name, capacity }
    }
}

impl PartialEq for RnResource {
    fn eq(&self, other : &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for RnResource {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Eq for RnResource {}

impl Ord for RnResource {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.id == other.id {
            return Ordering::Equal;
        } else if self.id < other.id {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }
}
