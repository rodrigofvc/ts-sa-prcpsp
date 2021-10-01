use crate::prcpsp::activity::Activity as Activity;
use crate::prcpsp::rn_resource::RnResource as RnResource;

#[derive(Debug,Clone)]
pub struct Project {
    pub activities : Vec<Activity>,
    pub resources : Vec<RnResource>
}

impl Project {
    pub fn new(activities : Vec<Activity>, resources: Vec<RnResource>) -> Project {
        Project { activities, resources }
    }
}
