
pub trait State {
    fn get_neighbor(&mut self) -> (u32, usize);
    fn set_neighbor(&mut self, movement: usize);
    fn get_cost(&self) -> u32;
    fn to_string(&self) -> String;
}
