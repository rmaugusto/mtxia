use bevy::{
    ecs::{Query, Res},
    prelude::Transform,
};

use crate::{
    fish::{self, Fish},
    sensor::SensorSet,
    shared::config::Config,
};

use super::FishAi;

fn net(vs: &nn::Path, in_count: usize) -> impl Module {
    nn::seq()
        // .add(nn::linear(vs, in_count as i64, 1, Default::default()))
        // .add_fn(|xs| xs.sigmoid())
        .add(nn::linear(
            vs,
            in_count as i64,
            6 as i64,
            LinearConfig {
                bias: true,
                ..Default::default()
            },
        ))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(
            vs,
            6 as i64,
            2 as i64,
            LinearConfig {
                bias: true,
                ..Default::default()
            },
        ))
        .add_fn(|xs| xs.relu())
}

pub(crate) fn forward_system(
    config: Res<Config>,
    mut q_fishes: Query<(&mut Fish, &SensorSet, &FishAi, &mut Transform)>,
) {
    for (mut fish, ss, fai, mut t) in q_fishes.iter_mut() {
        // println!("{:?}", fai.var_store);
        let mut other_input_data = vec![fish.energy, config.fish.speed];
        let mut other_input_data = vec![config.fish.speed];

        let net = net(
            &fai.var_store.root(),
            ss.sensors.len() + other_input_data.len(),
        );
        let distances: Vec<f32> = ss.sensors.iter().map(|s| s.distance).collect();

        other_input_data.extend(distances);

        let input = Tensor::of_slice(&other_input_data);
        let res = net.forward(&input);

        if res.double_value(&[0]) > 0f64 {
            fish::moviment_system::turn_fish(&fish::TurnFish::LEFT, &mut t, &mut fish);
        }

        if res.double_value(&[1]) > 0f64 {
            fish::moviment_system::turn_fish(&fish::TurnFish::RIGHT, &mut t, &mut fish);
        }
    }
}
