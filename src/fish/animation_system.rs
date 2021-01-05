use bevy::{
    core::{Time, Timer},
    ecs::{Query, Res},
    sprite::TextureAtlasSprite,
};

use super::{AnimationEntity, AnimationTimer, Fish};

pub(crate) fn animate_fish(
    time: Res<Time>,
    mut query: Query<(
        &Fish,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut AnimationEntity,
    )>,
) {
    for (fish, mut ani_timer, mut sprite, mut ae) in query.iter_mut() {
        ani_timer.timer.tick(time.delta_seconds);

        //Fish with 30% or less of energy change color
        if ani_timer.timer.finished {
            let min_index = match ((fish.energy / fish.max_energy) * 100f32) as i32 {
                0..=30 => 0,
                _ => 6,
            };
            let max_index = min_index + 2;

            if ae.reverse_index {
                sprite.index -= 1;
            } else {
                sprite.index += 1;
            }

            if sprite.index <= min_index {
                ae.reverse_index = false;
            } else if sprite.index >= max_index {
                ae.reverse_index = true;
            }
        }
    }
}
