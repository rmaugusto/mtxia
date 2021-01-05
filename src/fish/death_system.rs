use std::{
    fs,
    path::Path,
    time::{Duration, Instant},
};

use bevy::{ecs::prelude::*, prelude::DespawnRecursiveExt};

use crate::{
    ai::BEST_RESULTS_SAVED_COUNT,
    shared::{
        config::{Config, ModeEnum},
        gamedata::GameData,
    },
};

use super::{save_summary, Fish, FishAlive};

pub fn send_fish_death_by_energy(config: Res<Config>, mut query: Query<(Entity, &mut Fish)>) {
    for (_, mut fish) in query.iter_mut() {
        if fish.alive && fish.energy <= 0.0 && config.fish.energy != -1f32 {
            fish.die();
        }
    }
}

pub fn poll_dead_fish(mut gd: ResMut<GameData>, config: Res<Config>) {
    if gd.died_fishes.len() as i32 == config.fish.count {
        gd.died_fishes
            .sort_by(|f1, f2| f1.fitness().partial_cmp(&f2.fitness()).unwrap());
        gd.died_fishes.reverse();

        let mut gd_best_time: Option<Duration> = gd.best_time;
        let mut gd_create_generation = gd.create_generation;
        let gd_current_generation = gd.current_generation;

        let mut i: usize = 0;
        for f in &mut gd.died_fishes {
            //Last fish died
            if i <= BEST_RESULTS_SAVED_COUNT as usize {
                let result_dir = format!("{}{}", config.ai.state_path, f.brain.get_name());

                //Create result dir if not exists
                fs::create_dir_all(&result_dir);

                let best_str = format!("{}/{}_pos.yaml", result_dir, i);
                let best_path = Path::new(&best_str);

                if config.ai.mode == ModeEnum::LEARN {
                    f.brain.save(best_path);
                }

                if i == 0 {
                    gd_best_time = Some(f.create_at.elapsed());

                    let date = chrono::offset::Local::now();
                    println!(
                        "[{}] - Gen {}\tBest time: {:?}\tidx: {:>6}\tDistance: {}\tEnergy: {}\tFitness: {}\t",
                        date.format("[%H:%M:%S"),
                        gd_current_generation,
                        gd_best_time.unwrap().as_secs_f32(),
                        f.index,
                        f.distance,
                        f.energy,
                        f.fitness()
                    );

                    save_summary::save_summary(gd_current_generation, &config, f);
                }
            }

            i += 1;
        }

        gd.best_time = gd_best_time;
        gd.create_generation = true;
        gd.died_fishes.clear();
    }
}

pub fn remove_dead_fish(
    mut commands: Commands,
    q_fish: Query<Without<Fish, (Entity, &FishAlive)>>,
) {
    for (e, _) in q_fish.iter() {
        commands.despawn_recursive(e);
    }
}

pub fn recover_dead_fish(mut_world: &mut World, resources: &mut Resources) {
    loop {
        let mut query_fish = mut_world
            .query::<(Entity, &Fish, &FishAlive)>()
            .filter(|(_, f, _)| !f.alive);

        let has_next = query_fish.next();
        if has_next.is_some() {
            let res = has_next.unwrap();
            let removed_fish = mut_world.remove_one::<Fish>(res.0);

            if removed_fish.is_ok() {
                let fish = removed_fish.unwrap();
                let gd = resources.get_mut::<GameData>();
                gd.unwrap().died_fishes.push(fish);
            }
        } else {
            break;
        }
    }
}
