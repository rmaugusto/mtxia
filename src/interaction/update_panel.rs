use std::fmt::format;

use bevy::ecs::{Query, Res};
use bevy::ui::widget::Text;

use crate::{fish::Fish, sensor::SensorSet, shared::gamedata::GameData};

use super::TextRef;

pub(crate) fn text_update_system(
    gd: Res<GameData>,
    mut q_text: Query<(&mut Text, &TextRef)>,
    q_fishes: Query<(&Fish, &SensorSet)>,
) {
    let mut text_fill: Vec<String> = Vec::new();

    //Clean all texts
    for (mut text, _) in q_text.iter_mut() {
        text.value = "".into();
    }

    text_fill.push(String::from(format!("{: ^25}",".:: Game info ::.")));
    text_fill.push(String::from(format!(
        "Generation: {}",
        gd.current_generation
    )));
    text_fill.push(String::from(format!("Alive fishes: {}", q_fishes.iter().len())));
    text_fill.push(format!(
        "Last Best time: {}",
        gd.best_time.unwrap_or_default().as_millis()
    ));

    let mut best_fishes = q_fishes.iter().map(|(f, ss)| f).collect::<Vec<&Fish>>();
    best_fishes.sort_by(|f1, f2| f2.energy.partial_cmp(&f1.energy).unwrap());
    best_fishes.truncate(5);

    (0..5).into_iter().for_each(|i| {
        if best_fishes.get(i).is_some() {
            let tf = best_fishes.get(i).unwrap();

            text_fill.push(String::from(format!(
                "{}o energy: {:.2} - {}",
                i + 1,
                tf.energy,
                tf.index.to_string()
            )));
        } else {
            text_fill.push(format!("{}o energy: {:.2}", i + 1, 0f32));
        }
    });

    let sel_fish = q_fishes.iter().filter(|(f, _)| f.selected).last();

    if sel_fish.is_some() {
        let (fish, ss) = sel_fish.unwrap();

        // let min_dist = ss
        //     .sensors
        //     .iter()
        //     .min_by(|a, b| {
        //         a.distance
        //             .partial_cmp(&b.distance)
        //             .expect("Distance compare error")
        //     })
        //     .map(|s| s.distance)
        //     .expect("Distance error");

        text_fill.push(String::from(".:: Selected item ::."));
        text_fill.push(format!("Idx: {}", fish.index.to_string()));
        text_fill.push(format!("Energy: {:.2}", fish.energy));

        ss.sensors.iter().enumerate().for_each(|(idx,s)|{
            text_fill.push(format!("{}o sensor: {:.2}", idx+1, s.distance));
        });

    }

    //Fill texts with new content
    for (mut text, t_ref) in q_text.iter_mut() {
        if text_fill.get(t_ref.index).is_some() {
            text.value = text_fill[t_ref.index].clone();
        }
    }
}