pub trait State {
    fn get_neighbor(&mut self) -> (u32, usize, Vec<u32>);
    fn set_neighbor(&mut self, movement: usize);
    fn get_cost(&self) -> u32;
    fn to_string(&self) -> String;
    fn to_file(&self) -> String;
}
