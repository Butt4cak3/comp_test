pub mod health;
pub mod weapon;

pub trait HasComponent<T> {
    fn get_component(&self) -> &T;
    fn get_component_mut(&mut self) -> &mut T;
}
