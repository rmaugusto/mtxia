use bevy::{
    ecs::IntoQuerySystem,
    prelude::{stage, AppBuilder, Plugin},
};
use bevy::{prelude::*, type_registry::TypeRegistry};

use super::*;

pub struct FishPlugin;

impl Plugin for FishPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(death_system::recover_dead_fish.thread_local_system());
        app.add_system(death_system::remove_dead_fish.system());
        app.add_system(death_system::poll_dead_fish.system());
        app.add_system(creation_system::create_fishes.system());
        app.add_system(animation_system::animate_fish.system());
        app.add_system_to_stage("MOVE_FISH",moviment_system::move_fish_forward.system());
        app.add_system_to_stage(
            "COLLISION_SYSTEM",
            collision_system::detect_fish_collision.system(),
        );
        app.add_system(death_system::send_fish_death_by_energy.system());
    }
}
