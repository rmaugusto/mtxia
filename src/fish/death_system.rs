use std::{fs, path::Path, time::Instant};

use bevy::{ecs::QueryError, prelude::*};

use crate::{
    ai::{FishAi, BEST_RESULTS_SAVED_COUNT},
    shared::{
        config::{Config, ModeEnum},
        gamedata::GameData,
    },
};

use super::Fish;

// use super::{Fish, FishDiedEvent};

// #[derive(Default)]
// pub struct FishDiedStateEvent {
// reader: EventReader<FishDiedEvent>,
// }

pub fn send_fish_death_by_energy(mut query: Query<(Entity, &mut Fish)>) {
    for (_, mut fish) in query.iter_mut() {
        if fish.energy <= 0.0 {
            fish.died_at = Some(Instant::now());
        }
    }
}

pub fn receive_fish_death(
    mut commands: Commands,
    mut gd: ResMut<GameData>,
    config: Res<Config>,
    mut q_fish: Query<(Entity, &mut Fish, &FishAi)>,
) {

    let mut fish_count = q_fish.iter_mut().len();

    let mut fishes_died: Vec<(Entity, Mut<Fish>, &FishAi)> = q_fish
        .iter_mut()
        .filter(|(_, f, _)| f.died_at.is_some())
        .collect();

    fishes_died.sort_by(|(_, f1, _), (_, f2, _)| {
        f2.died_at
            .unwrap()
            .partial_cmp(&f1.died_at.unwrap())
            .unwrap()
    });

    let died_count = fishes_died.len();

    let mut remaining_fishes = fish_count;

    fishes_died.iter_mut().for_each(|(e, f, _)| {

        //Last fish died
        if remaining_fishes <= BEST_RESULTS_SAVED_COUNT as usize {
            let result_dir = format!("{}{}", config.general.state_path, f.ai_processor.get_name());

            //Create result dir if not exists
            fs::create_dir_all(&result_dir);

            let best_str = format!("{}/{}_pos.yaml", result_dir, remaining_fishes);
            let best_path = Path::new(&best_str);

            gd.best_time = Some(f.create_at.elapsed());

            if config.ai.mode == ModeEnum::LEARN {
                f.ai_processor.save(best_path);
            }

            if remaining_fishes == 1 {
                let date = chrono::offset::Local::now();
                println!(
                    "[{}] - Gen {}, Best time: {:?}, dead by {}",
                    date.format("[%H:%M:%S"),
                    gd.current_generation,
                    gd.best_time.unwrap().as_millis(),
                    if f.energy <= 0f32 {
                        "energy"
                    } else {
                        "collision"
                    }
                );
                gd.create_generation = true;
            }
        }

    //TODO GET FISH REMOVED        
        let fish_removed = commands.remove::<Fish>(*e);
        commands.despawn_recursive(*e);

        remaining_fishes -= 1;
    });
}
