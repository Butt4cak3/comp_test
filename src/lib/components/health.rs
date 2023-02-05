use super::HasComponent;

#[derive(Debug)]
pub struct Health(pub i32);

pub trait HealthTrait {
    fn health(&self) -> i32;
    fn hurt(&mut self, amount: i32);
    fn heal(&mut self, amount: i32);
}

impl<T: HasComponent<Health>> HealthTrait for T {
    fn health(&self) -> i32 {
        self.get_component().0
    }

    fn hurt(&mut self, amount: i32) {
        self.get_component_mut().0 -= amount;
    }

    fn heal(&mut self, amount: i32) {
        self.get_component_mut().0 += amount;
    }
}
