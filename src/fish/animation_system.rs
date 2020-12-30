use bevy::{
    core::{Time, Timer},
    ecs::{Query, Res},
    sprite::TextureAtlasSprite,
};

use super::{AnimationEntity, AnimationTimer};

pub(crate) fn animate_fish(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &mut AnimationEntity)>,
) {
    for (mut ani_timer, mut sprite, mut ae) in query.iter_mut() {
        ani_timer.timer.tick(time.delta_seconds);

        if ani_timer.timer.finished {
            if ae.reverse_index {
                sprite.index -= 1;
            } else {
                sprite.index += 1;
            }

            if sprite.index == 0 {
                ae.reverse_index = false;
            } else if sprite.index >= 2 {
                ae.reverse_index = true;
            }
        }
    }
}
