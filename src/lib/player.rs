use comp_macros::HasComponent;

use crate::attack::AttackTrait;
use crate::components::health::Health;
use crate::components::weapon::Weapon;
use crate::components::HasComponent;

#[derive(Debug, HasComponent)]
pub struct Player {
    #[component]
    pub health_comp: Health,
    #[component]
    pub weapon_comp: Weapon,
}

impl Player {
    pub fn new(health_comp: Health, weapon_comp: Weapon) -> Self {
        Self {
            health_comp,
            weapon_comp,
        }
    }
}

impl AttackTrait for Player {}
