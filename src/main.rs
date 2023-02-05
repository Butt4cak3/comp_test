use comp_test::attack::AttackTrait;
use comp_test::components::health::{Health, HealthTrait};
use comp_test::components::weapon::Weapon;
use comp_test::enemy::Enemy;
use comp_test::player::Player;
use comp_test::weapon::WeaponType;

fn main() {
    let mut player = Player::new(
        Health(10),
        Weapon {
            weapon: WeaponType::Sword,
            power: 5,
        },
    );

    let mut enemy = Enemy::new(
        Health(10),
        Weapon {
            weapon: WeaponType::Club,
            power: 2,
        },
    );

    player.hurt(5);
    enemy.hurt(8);

    enemy.parry(&mut player);

    println!("{:?}\n {:?}", player, enemy);
}
