use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub fn weapon_type_from_u8(n: u8) -> WeaponTypes {
    match n {
        0 => WeaponTypes::LaserDouble,
        1 => WeaponTypes::LaserBeam,
        2 => WeaponTypes::LaserPulse,
        3 => WeaponTypes::ProjectileBurstFire,
        4 => WeaponTypes::ProjectileRapidFire,
        5 => WeaponTypes::ProjectileCannonFire,
        6 => WeaponTypes::Missile,
        7 => WeaponTypes::Rockets,
        8 => WeaponTypes::Mine,
        9 => WeaponTypes::LaserSword,
        _ => WeaponTypes::LaserDouble,
    }
}

pub fn get_next_weapon_type(weapon_type: WeaponTypes) -> Option<WeaponTypes> {
    match weapon_type {
        WeaponTypes::LaserDouble => Some(WeaponTypes::ProjectileRapidFire),
        WeaponTypes::ProjectileRapidFire => Some(WeaponTypes::Missile),
        WeaponTypes::Missile => Some(WeaponTypes::LaserBeam),
        WeaponTypes::LaserBeam => Some(WeaponTypes::ProjectileCannonFire),
        WeaponTypes::ProjectileCannonFire => Some(WeaponTypes::Rockets),
        WeaponTypes::Rockets => Some(WeaponTypes::LaserPulse),
        WeaponTypes::LaserPulse => Some(WeaponTypes::ProjectileBurstFire),
        WeaponTypes::ProjectileBurstFire => Some(WeaponTypes::Mine),
        WeaponTypes::Mine => Some(WeaponTypes::LaserSword),
        WeaponTypes::LaserSword => None
    }
}


pub fn update_weapon_properties(weapon: &mut Weapon, weapon_type: WeaponTypes) {
    let (weapon_type,
        heat_seeking,
        heat_seeking_agility,
        weapon_cooldown, 
        burst_shot_limit,
        burst_cooldown,
        weapon_shot_speed,
        damage,
        shield_damage_pct,
        armor_damage_pct,
        piercing_damage_pct,
        health_damage_pct) = build_standard_weapon(weapon_type.clone());

    weapon.weapon_type = weapon_type;
    weapon.heat_seeking = heat_seeking;
    weapon.heat_seeking_agility = heat_seeking_agility;
    weapon.weapon_cooldown_reset = weapon_cooldown;
    weapon.burst_shot_limit = burst_shot_limit;
    weapon.burst_cooldown_reset = burst_cooldown;
    weapon.weapon_shot_speed = weapon_shot_speed;
    weapon.damage = damage;
    weapon.shield_damage_pct = shield_damage_pct;
    weapon.armor_damage_pct = armor_damage_pct;
    weapon.piercing_damage_pct = piercing_damage_pct;
    weapon.piercing_damage_pct = piercing_damage_pct;
    weapon.health_damage_pct = health_damage_pct;
}



pub fn build_standard_weapon(weapon_type: WeaponTypes) -> (
    WeaponTypes, bool, f32, f32, u32, f32, f32, f32, f32, f32, f32, f32
    ) {
    let (weapon_shot_speed, damage, weapon_cooldown, 
            piercing_damage_pct, 
            shield_damage_pct, armor_damage_pct, 
            health_damage_pct,
        ) = match weapon_type.clone()
    {                                      //speed      dmg     cooldwn pierce% shield%   armor%    health%
        WeaponTypes::LaserDouble =>         (400.0,     25.0,   0.4,    0.0,   120.0,     60.0,     100.0),
        WeaponTypes::LaserBeam =>           (2800.0,    0.3,    0.005,  0.0,   120.0,     60.0,     100.0),
        WeaponTypes::LaserPulse =>          (400.0,     12.0,   0.75,   0.0,   120.0,     60.0,     100.0),
        WeaponTypes::ProjectileBurstFire => (250.0,     12.0,   0.15,   0.0,    80.0,     90.0,     100.0),
        WeaponTypes::ProjectileRapidFire => (250.0,     3.0,    0.10,   0.0,    80.0,     90.0,     100.0),
        WeaponTypes::ProjectileCannonFire =>(700.0,     50.0,   0.9,    0.0,    80.0,     90.0,     100.0),
        WeaponTypes::Missile =>             (100.0,     50.0,   2.5,    10.0,   75.0,     75.0,     100.0),
        WeaponTypes::Rockets =>             (250.0,     50.0,   0.8,    10.0,   75.0,     75.0,     100.0),
        WeaponTypes::Mine =>                (0.0,       50.0,   2.5,    10.0,   75.0,     75.0,     100.0),
        WeaponTypes::LaserSword =>          (0.0,       1.0,    1000.0,    50.0,    75.0,     75.0,      100.0),
    };
    
    let burst_cooldown;
    let burst_shot_limit; 
    let heat_seeking_agility;

    if weapon_type.clone() == WeaponTypes::LaserPulse {
        burst_cooldown = 0.1 as f32;
        burst_shot_limit = 2 as u32;
    }
    else if weapon_type.clone() == WeaponTypes::ProjectileBurstFire{
        burst_cooldown = 0.1 as f32;
        burst_shot_limit = 2 as u32;
    }
    else if weapon_type.clone() == WeaponTypes::Rockets{
        burst_cooldown = 0.25 as f32;
        burst_shot_limit = 4 as u32;
    }
    else {
        burst_cooldown = weapon_cooldown.clone();
        burst_shot_limit = 1 as u32;
    };
    
    let heat_seeking;
    if weapon_type.clone() == WeaponTypes::Missile {
        heat_seeking = true;
        heat_seeking_agility = 1.0;
    }
    else {
        heat_seeking = false;
        heat_seeking_agility = 0.0;
    }
    
    (weapon_type,
        heat_seeking,
        heat_seeking_agility,
        weapon_cooldown, 
        burst_shot_limit,
        burst_cooldown,
        weapon_shot_speed,
        damage,
        shield_damage_pct,
        armor_damage_pct,
        piercing_damage_pct,
        health_damage_pct,)
    }




#[derive(Clone, Debug, PartialEq)]
pub enum WeaponTypes {
    LaserBeam,
    LaserPulse,
    LaserDouble,
    ProjectileRapidFire,
    ProjectileBurstFire,
    ProjectileCannonFire,
    Mine,
    Missile,
    Rockets,
    LaserSword,
}


#[derive(Clone)]
pub struct Weapon {
    pub x: f32,
    pub y: f32,
    pub aim_angle: f32,
    pub weapon_type: WeaponTypes,
    pub heat_seeking: bool,
    pub heat_seeking_agility: f32,
    pub weapon_cooldown_timer: f32,
    pub weapon_cooldown_reset: f32,
    pub burst_shots: u32,
    pub burst_shot_limit: u32,
    pub burst_cooldown_reset: f32,
    pub damage: f32,
    pub weapon_shot_speed: f32,
    pub shield_damage_pct: f32,
    pub armor_damage_pct: f32,
    pub piercing_damage_pct: f32,
    pub health_damage_pct: f32,
}

impl Component for Weapon {
    type Storage = DenseVecStorage<Self>;
}

impl Weapon {
    pub fn new(weapon_type: WeaponTypes, 
        heat_seeking: bool,
        heat_seeking_agility: f32,
        weapon_cooldown: f32, 
        burst_shot_limit: u32, 
        burst_cooldown: f32,
        weapon_shot_speed: f32,
        damage: f32,
        shield_damage_pct: f32,
        armor_damage_pct: f32,
        piercing_damage_pct: f32,
        health_damage_pct: f32,
    ) -> Weapon {

        Weapon {
            x: 0.0,
            y: 0.0,
            aim_angle: 0.0,
            weapon_type,
            heat_seeking,
            heat_seeking_agility,
            weapon_cooldown_timer: -1.0,
            weapon_cooldown_reset: weapon_cooldown,
            burst_shots: 0,
            burst_shot_limit: burst_shot_limit,
            burst_cooldown_reset: burst_cooldown,
            damage: damage,
            weapon_shot_speed: weapon_shot_speed,
            shield_damage_pct: shield_damage_pct,
            armor_damage_pct: armor_damage_pct,
            piercing_damage_pct: piercing_damage_pct,
            health_damage_pct: health_damage_pct,
        }
    }
}


#[derive(Clone)]
pub struct WeaponFire {
    pub width: f32,
    pub height: f32,
    pub dx: f32,
    pub dy: f32,
    pub spawn_x: f32,
    pub spawn_y: f32,
    pub spawn_angle: f32,
    pub weapon_shot_speed: f32,
    pub owner_player_id: usize,
    pub damage: f32,
    pub shield_damage_pct: f32,
    pub armor_damage_pct: f32,
    pub piercing_damage_pct: f32,
    pub health_damage_pct: f32,
    pub heat_seeking: bool,
    pub heat_seeking_agility: f32,
    pub weapon_type: WeaponTypes,
}

impl Component for WeaponFire {
    type Storage = DenseVecStorage<Self>;
}


impl WeaponFire {
    pub fn new(weapon_type: WeaponTypes, 
        owner_player_id: usize,
        heat_seeking: bool,
        heat_seeking_agility: f32,
        weapon_shot_speed: f32,
        damage: f32,
        shield_damage_pct: f32,
        armor_damage_pct: f32,
        piercing_damage_pct: f32,
        health_damage_pct: f32,
    ) -> WeaponFire {

        let (width, height) = match weapon_type.clone()
        {                                      
            WeaponTypes::LaserDouble =>         (3.0, 6.0),
            WeaponTypes::LaserBeam =>           (1.0, 20.0),
            WeaponTypes::LaserPulse =>          (1.0, 3.0),
            WeaponTypes::ProjectileBurstFire => (1.0, 4.0),
            WeaponTypes::ProjectileRapidFire => (1.0, 2.0),
            WeaponTypes::ProjectileCannonFire =>(2.0, 3.0),
            WeaponTypes::Missile =>             (3.0, 5.0),
            WeaponTypes::Rockets =>             (5.0, 3.0),
            WeaponTypes::Mine =>                (3.0, 3.0),
            WeaponTypes::LaserSword =>          (3.0, 4.0),
        };

        WeaponFire {
            width,
            height,
            dx: 0.0,
            dy: 0.0,
            spawn_x: 0.0,
            spawn_y: 0.0,
            spawn_angle: 0.0,
            owner_player_id,
            damage: damage,
            weapon_shot_speed: weapon_shot_speed,
            shield_damage_pct: shield_damage_pct,
            armor_damage_pct: armor_damage_pct,
            piercing_damage_pct: piercing_damage_pct,
            health_damage_pct: health_damage_pct,
            heat_seeking,
            heat_seeking_agility,
            weapon_type,
        }
    }
}
