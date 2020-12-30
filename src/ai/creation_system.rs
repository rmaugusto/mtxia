use std::path::Path;

use bevy::ecs::{Commands, Entity, Query, Res, Without};

use crate::{fish::Fish, shared::config::{Config, ModeEnum}};

use super::{BEST_RESULTS_SAVED_COUNT, FishAi};

pub(crate) fn create_ai(
    mut commands: Commands,
    config: Res<Config>,
    mut q_unlocked_objs: Query<Without<FishAi, (Entity, &mut Fish)>>,
) {
    for (e, mut f) in q_unlocked_objs.iter_mut() {
        let result_dir = format!("{}{}", config.general.state_path, f.ai_processor.get_name());

        let mut file_idx = (f.index % (BEST_RESULTS_SAVED_COUNT as i32 )) as usize;

        // If Ai in run mode use only the best result
        if config.ai.mode == ModeEnum::RUN {
            file_idx = 0;
        }

        let best_str = format!("{}/{}_pos.yaml", result_dir, file_idx + 1);
        let best_path = Path::new(&best_str);

        if best_path.exists() {
            // Load best fish stored if exists
            f.ai_processor.load(best_path);
        }else{
            // Generate random data if not exists
            f.ai_processor.random_weights();
        }
        
        if f.index >= BEST_RESULTS_SAVED_COUNT && config.ai.mode == ModeEnum::LEARN {
            // Apply random value over loaded data
            f.ai_processor.modify_random_weights();
        }

        commands.insert_one(e, FishAi {});
    }
}
