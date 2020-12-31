use std::{fs, path::Path, time::{Duration, Instant}};

use bevy::{ecs::QueryError, prelude::*};

use crate::{
    ai::{FishAi, BEST_RESULTS_SAVED_COUNT},
    shared::{
        config::{Config, ModeEnum},
        gamedata::GameData,
    },
};

use super::{Fish, FishMarkedToRemove};

pub fn send_fish_death_by_energy(mut query: Query<(Entity, &mut Fish)>) {
    for (_, mut fish) in query.iter_mut() {
        if fish.energy <= 0.0 {
            fish.died_at = Some(Instant::now());
        }
    }
}

pub fn poll_dead_fish(mut gd: ResMut<GameData>, config: Res<Config>) {
    if gd.died_fishes.len() as i32 == config.fish.count {
        gd.died_fishes
            .sort_by(|f1, f2| f2.performed.partial_cmp(&f1.performed).unwrap());


        let mut gd_best_time:Option<Duration> = gd.best_time;
        let mut gd_create_generation = gd.create_generation;
        let gd_current_generation = gd.current_generation;

        let mut i:usize = 0;
        for f in &mut gd.died_fishes {
            
            //Last fish died
            if i <= BEST_RESULTS_SAVED_COUNT as usize {
                let result_dir =
                    format!("{}{}", config.general.state_path, f.ai_processor.get_name());

                //Create result dir if not exists
                fs::create_dir_all(&result_dir);

                let best_str = format!("{}/{}_pos.yaml", result_dir, i);
                let best_path = Path::new(&best_str);

                gd_best_time = Some(f.create_at.elapsed());

                if config.ai.mode == ModeEnum::LEARN {
                    f.ai_processor.save(best_path);
                }
                if i == 1 {
                    let date = chrono::offset::Local::now();
                    println!(
                        "[{}] - Gen {}, Best time: {:?}, dead by {}",
                        date.format("[%H:%M:%S"),
                        gd_current_generation,
                        gd_best_time.unwrap().as_millis(),
                        if f.energy <= 0f32 {
                            "energy"
                        } else {
                            "collision"
                        }
                    );
                    gd_create_generation = true;
                }
            }

            i += 1;
        }

        gd.best_time = gd_best_time;
        gd.create_generation = gd_create_generation;
        gd.died_fishes.clear();


        // gd.died_fishes.iter_mut().enumerate().for_each(|(i, f)| {
        //     //Last fish died
        //     if i <= BEST_RESULTS_SAVED_COUNT as usize {
        //         let result_dir =
        //             format!("{}{}", config.general.state_path, f.ai_processor.get_name());

        //         //Create result dir if not exists
        //         fs::create_dir_all(&result_dir);

        //         let best_str = format!("{}/{}_pos.yaml", result_dir, i);
        //         let best_path = Path::new(&best_str);

        //         gd.best_time = Some(f.create_at.elapsed());

        //         if config.ai.mode == ModeEnum::LEARN {
        //             f.ai_processor.save(best_path);
        //         }
        //         if i == 1 {
        //             let date = chrono::offset::Local::now();
        //             println!(
        //                 "[{}] - Gen {}, Best time: {:?}, dead by {}",
        //                 date.format("[%H:%M:%S"),
        //                 gd.current_generation,
        //                 gd.best_time.unwrap().as_millis(),
        //                 if f.energy <= 0f32 {
        //                     "energy"
        //                 } else {
        //                     "collision"
        //                 }
        //             );
        //             gd.create_generation = true;
        //         }
        //     }
        // });
    }
}

pub fn remove_dead_fish(mut commands: Commands, q_fish: Query<Without<Fish, (Entity, &FishAi)>>) {
    for (e, _) in q_fish.iter() {
        commands.despawn_recursive(e);
    }
}

pub fn recover_dead_fish(
    mut_world: &mut World,
    resources: &mut Resources, // mut commands: Commands,
                               // mut gd: ResMut<GameData>,
                               // config: Res<Config>,
                               // mut q_fish: Query<(Entity, &Fish)>
) {
    loop {
        let mut query_fish = mut_world
            .query::<(Entity, &Fish)>()
            .filter(|(e, f)| f.died_at.is_some());

        let has_next = query_fish.next();
        if has_next.is_some() {
            let res = has_next.unwrap();
            let removed_fish = mut_world.remove_one::<Fish>(res.0);

            if removed_fish.is_ok() {
                let gd = resources.get_mut::<GameData>();
                gd.unwrap().died_fishes.push(removed_fish.unwrap());
            }
        } else {
            break;
        }
    }

    // mut_world.spawn((1,2));
    // mut_world.spawn((1,2));

    // let q = mut_world.query_mut::<(Entity, &Fish)>();

    // for x in q {
    // mut_world.despawn(x.0);
    // mut_world.spawn((1,2));
    // }

    // let mut fishes_died: Vec<(Entity, &Fish)> = mut_world.query_mut::<(Entity, &Fish)>()
    //     .filter(|(e, f)| f.died_at.is_some())
    //     .collect::<Vec<_>>();

    // xx(&mut mut_world, &fishes_died);

    // let mut fishes_died: Vec<(Entity, &Fish)> = mut_world.query::<(Entity, &Fish)>()
    //     .filter(|(e, f)| f.died_at.is_some())
    //     .collect();
    // {
    //     mut_world.remove_one::<Fish>(fishes_died.get(0).unwrap().0);
    // }

    // for x in 0..fishes_died.len() {
    //     mut_world.remove_one::<Fish>(fishes_died.get(0).unwrap().0);

    // }

    // let a = Entity::new(123);
    // let mut idx =0;
    // loop {
    // if fishes_died.get(0).is_none() {
    // continue;
    // }
    // idx +=1 ;
    // }

    // let q_fish = world.query::<(Entity, &Fish)>().for_each(|a|{
    //     let x = a.0;
    //     let f= mut_world.remove_one::<Fish>(x);
    // });

    // let mut fishes_died: Vec<(Entity, &Fish)> = world.query::<(Entity, &Fish)>()
    //     .filter(|(e, f)| f.died_at.is_some())
    //     .collect();

    // (0..10).for_each(|_|{
    //     let f = || {
    //         let f= mut_world.remove_one::<Fish>(fishes_died.first().unwrap().0);

    //     };
    // });

    // let f= mut_world.remove_one::<Fish>(fishes_died.first().unwrap().0);

    // fishes_died.into_iter().for_each(|(e, f)| {
    //     let f= _world.remove_one::<Fish>(*e);
    // });

    // let mut fishes_died: Vec<(&Entity, Mut<Fish>, &FishAi)> = q_fish
    //     .iter_mut()
    //     .filter(|(_, f, _)| f.died_at.is_some())
    //     .collect();

    // let a = fishes_died.get(0).unwrap();
    // let x = q_fish.get_component_mut::<Mut<Fish>>(*a.0);

    // fishes_died.into_iter().for_each(|(e, f, _)| {
    //     // let x = q_fish.get_component_mut::<Fish>(*e);
    //     commands.despawn_recursive(*e);
    // });

    // q_fish
    //     .iter_mut()
    //     .filter(|(_, f, _)| f.died_at.is_some())
    //     .for_each(|(e,mut _f,_fai)|{

    //         let ff = q_fish.get_component_mut::<Fish>(e);

    //         let xx = ff.unwrap();

    //         let a = commands.despawn_recursive(e);
    //         let b = 3;

    //     });

    // let mut fish_count = q_fish.iter_mut().len();

    // let mut fishes_died: Vec<(Entity, Mut<Fish>, &FishAi)> = q_fish
    //     .iter_mut()
    //     .filter(|(_, f, _)| f.died_at.is_some())
    //     .collect();

    // fishes_died.sort_by(|(_, f1, _), (_, f2, _)| {
    //     f2.died_at
    //         .unwrap()
    //         .partial_cmp(&f1.died_at.unwrap())
    //         .unwrap()
    // });

    // let died_count = fishes_died.len();

    // let mut remaining_fishes = fish_count;

    // fishes_died.iter_mut().for_each(|(e, f, _)| {

    //     //Last fish died
    //     if remaining_fishes <= BEST_RESULTS_SAVED_COUNT as usize {
    //         let result_dir = format!("{}{}", config.general.state_path, f.ai_processor.get_name());

    //         //Create result dir if not exists
    //         fs::create_dir_all(&result_dir);

    //         let best_str = format!("{}/{}_pos.yaml", result_dir, remaining_fishes);
    //         let best_path = Path::new(&best_str);

    //         gd.best_time = Some(f.create_at.elapsed());

    //         if config.ai.mode == ModeEnum::LEARN {
    //             f.ai_processor.save(best_path);
    //         }
    //         if remaining_fishes == 1 {
    //             let date = chrono::offset::Local::now();
    //             println!(
    //                 "[{}] - Gen {}, Best time: {:?}, dead by {}",
    //                 date.format("[%H:%M:%S"),
    //                 gd.current_generation,
    //                 gd.best_time.unwrap().as_millis(),
    //                 if f.energy <= 0f32 {
    //                     "energy"
    //                 } else {
    //                     "collision"
    //                 }
    //             );
    //             gd.create_generation = true;
    //         }
    //     }

    // //TODO GET FISH REMOVED
    //     // let fish_removed = commands.remove::<Fish>(*e);
    //     commands.despawn_recursive(*e);

    //     remaining_fishes -= 1;
    // });
}
