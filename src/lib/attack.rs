use crate::components::health::HealthTrait;
use crate::components::weapon::WeaponTrait;

pub trait AttackTrait: HealthTrait + WeaponTrait {
    fn attack<T>(&self, other: &mut T)
    where
        T: HealthTrait,
    {
        other.hurt(self.power());
    }

    fn parry<T>(&mut self, other: &mut T)
    where
        T: HealthTrait + WeaponTrait,
    {
        let attack_power = other.power();
        let defence_power = self.power();
        let power_difference = (attack_power - defence_power).abs();

        if defence_power > attack_power {
            other.hurt(power_difference);
        } else {
            self.hurt(power_difference);
        }
    }
}
