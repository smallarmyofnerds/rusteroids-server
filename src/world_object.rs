pub trait WorldObject {
    fn get_id(&self) -> u64;
    fn destroy(&mut self);
    fn should_be_destroyed(&self) -> bool;
}
