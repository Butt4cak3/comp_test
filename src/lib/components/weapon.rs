use crate::weapon::WeaponType;

use super::HasComponent;

#[derive(Debug)]
pub struct Weapon {
    pub weapon: WeaponType,
    pub power: i32,
}

pub trait WeaponTrait {
    fn weapon(&self) -> &WeaponType;
    fn power(&self) -> i32;
}

impl<T: HasComponent<Weapon>> WeaponTrait for T {
    fn weapon(&self) -> &WeaponType {
        &self.get_component().weapon
    }

    fn power(&self) -> i32 {
        self.get_component().power
    }
}
