use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
};

use crate::audio::{play_bounce_sound, Sounds};
use log::debug;
use std::collections::HashMap;


use crate::components::{kill_restart_vehicle, Player, Vehicle};
use crate::rally::{
    vehicle_damage_model, BASE_COLLISION_DAMAGE, COLLISION_ARMOR_DAMAGE_PCT,
    COLLISION_HEALTH_DAMAGE_PCT, COLLISION_PIERCING_DAMAGE_PCT, COLLISION_SHIELD_DAMAGE_PCT,
};



#[derive(SystemDesc, Default)]
pub struct CollisionVehToVehSystem;

impl<'s> System<'s> for CollisionVehToVehSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Vehicle>,
        Read<'s, Time>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (mut transforms, players, mut vehicles,
            time, storage, sounds, audio_output): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        let mut collision_ids_map = HashMap::new();

        for (vehicle_1, player_1, vehicle_1_transform) in (&vehicles, &players, &transforms).join()
        {
            let vehicle_1_x = vehicle_1_transform.translation().x;
            let vehicle_1_y = vehicle_1_transform.translation().y;

            for (vehicle_2, player_2, vehicle_2_transform) in
                (&vehicles, &players, &transforms).join()
            {
                let vehicle_2_x = vehicle_2_transform.translation().x;
                let vehicle_2_y = vehicle_2_transform.translation().y;

                if (player_1.id != player_2.id) &&
                    (vehicle_1_x - vehicle_2_x).powi(2) + (vehicle_1_y - vehicle_2_y).powi(2)
                    < vehicle_1.width.powi(2)
                {
                    // let veh_hit_non_bounce_decel_pct: f32 = 0.35;
                    // let veh_hit_bounce_decel_pct: f32 = -veh_hit_non_bounce_decel_pct;

                    let sq_vel_diff = (vehicle_1.dx - vehicle_2.dx).powi(2)
                        + (vehicle_1.dy - vehicle_2.dy).powi(2);
                    let abs_vel_diff = sq_vel_diff.sqrt();

                    collision_ids_map.insert(player_1.id, abs_vel_diff);
                    collision_ids_map.insert(player_2.id, abs_vel_diff);

                    /*
                    let velocity_1_angle = vehicle_1.dy.atan2(vehicle_1.dx) - (PI/2.0); //rotate by PI/2 to line up with yaw angle
                    let velocity_1_x_comp = -velocity_1_angle.sin(); //left is -, right is +
                    let velocity_1_y_comp = velocity_1_angle.cos(); //up is +, down is -


                    let velocity_2_angle = vehicle_2.dy.atan2(vehicle_2.dx) - (PI/2.0); //rotate by PI/2 to line up with yaw angle
                    let velocity_2_x_comp = -velocity_2_angle.sin(); //left is -, right is +
                    let velocity_2_y_comp = velocity_2_angle.cos(); //up is +, down is -
                    */
                }
            }
        }

        for (vehicle, player, transform) in (&mut vehicles, &players, &mut transforms).join() {

            let collision_ids = collision_ids_map.get(&player.id);

            if let Some(collision_ids) = collision_ids {
                let v_diff = collision_ids;
                if vehicle.collision_cooldown_timer <= 0.0 {
                    debug!("Player {} has collided", player.id);

                    let damage: f32 = BASE_COLLISION_DAMAGE * v_diff;

                    if *v_diff > 1.0 {
                        play_bounce_sound(
                            &*sounds,
                            &storage,
                            audio_output.as_deref(),
                        );
                    }
                    vehicle.collision_cooldown_timer = 1.0;

                    let vehicle_destroyed: bool = vehicle_damage_model(
                        vehicle,
                        damage,
                        COLLISION_PIERCING_DAMAGE_PCT,
                        COLLISION_SHIELD_DAMAGE_PCT,
                        COLLISION_ARMOR_DAMAGE_PCT,
                        COLLISION_HEALTH_DAMAGE_PCT,
                    );

                    if vehicle_destroyed {
                        kill_restart_vehicle(player, vehicle, transform);
                    }

                //vehicle_1.dx *= veh_hit_bounce_decel_pct * velocity_1_x_comp.abs();
                //vehicle_1.dy *= veh_hit_bounce_decel_pct * velocity_1_y_comp.abs();

                //vehicle_2.dx *= veh_hit_bounce_decel_pct * velocity_2_x_comp.abs();
                //vehicle_2.dy *= veh_hit_bounce_decel_pct * velocity_2_y_comp.abs();
                } else {
                    vehicle.collision_cooldown_timer -= dt;
                }
            }
        }
    }
}
