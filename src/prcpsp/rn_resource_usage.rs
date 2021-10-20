use crate::prcpsp::rn_resource::RnResource as RnResource;

/**
*   Represents the units that actual activity is using
*   for a renewable resource.
*/
#[derive(Debug,Clone)]
pub struct RnResourceUsage {
    pub resource: RnResource,
    pub usage: u32,
}

impl RnResourceUsage {
    pub fn new(resource: RnResource, usage: u32) -> RnResourceUsage {
        RnResourceUsage { resource, usage }
    }
}
