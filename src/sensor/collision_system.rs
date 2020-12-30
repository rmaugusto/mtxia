use bevy::{
    prelude::*,
    tasks::{ComputeTaskPool, ParallelIterator},
};

use crate::{ground::{FloorTile, OtherTypeIntersection, HALF_TILE_SIZE, TILE_SIZE}, shared::{config::Config, gamedata::GameData}};

use super::{SensorDetectedEnum, SensorSet, SENSOR_REACH};

pub fn detect_sensor_collision(
    gd: Res<GameData>,
    pool: Res<ComputeTaskPool>,
    mut q_sensorset: Query<(&Transform, &mut SensorSet)>,
    q_floor: Query<(&Transform, &FloorTile, &OtherTypeIntersection)>,
) {
    q_sensorset.par_iter_mut(32).for_each(&pool, |(_, mut ss)| {
    // q_sensorset.iter_mut().for_each(|(_, mut ss)| {
        let obs_point = ss.observer_point;

        // q_floor.par_iter(32).for_each(&pool, |(t_floor, _, _)| {

        for (_, sensor) in ss.sensors.iter_mut().enumerate() {
            //Clean sensor
            sensor.detected = SensorDetectedEnum::NOTHING;
            sensor.distance = SENSOR_REACH;

            //Ignore observer_point because it is on sensor position
            let lx1 = sensor.end_point.x();
            let ly1 = sensor.end_point.y();
            let lx2 = obs_point.x();
            let ly2 = obs_point.y();

            // if lx2 <= gd.map_data.x1 && ly2 <= gd.map_data.y1 
            //     || lx2 >= gd.map_data.x2 && ly2 >= gd.map_data.x2 {
            //     // Object out of the map
            //     sensor.detected = SensorDetectedEnum::FLOOR;
            //     sensor.distance = 0f32;
            //     sensor.end_point = obs_point.clone();
            // } else{

                q_floor.iter().for_each(|(t_floor, _, _)| {
                // q_floor.par_iter(32).for_each(&pool, |(t_floor, _, _)| {
                    //Use floor position aligned to the center
                    let sx = t_floor.translation.x() - HALF_TILE_SIZE;
                    let sy = t_floor.translation.y() - HALF_TILE_SIZE;
                    let sw = TILE_SIZE;
                    let sh = TILE_SIZE;
    
                    // Optimize serach using only tiles near the observer
                    if inside_circle(obs_point.x(), obs_point.y(), SENSOR_REACH * 2f32, sx, sy) {
                        let contacts = line_square(lx1, ly1, lx2, ly2, sx, sy, sw, sh);
    
                        if !contacts.is_empty() {
                            contacts.iter().for_each(|(x, y)| {
                                let calc_dist = ((obs_point.x() - x).powf(2.0)
                                    + (obs_point.y() - y).powf(2.0))
                                .sqrt();
    
                                if calc_dist < sensor.distance {
                                    sensor.detected = SensorDetectedEnum::FLOOR;
                                    sensor.distance = calc_dist;
                                    sensor.end_point = Vec2::new(*x, *y);
                                }
                            });
                        }
                    }
                });                

            // }


        }
    });
}

fn inside_circle(center_x: f32, center_y: f32, radius: f32, x: f32, y: f32) -> bool {
    let dist = ((center_x - x).powf(2.0f32) + (center_y - y).powf(2.0)).sqrt();
    dist <= radius
}
/// Return all contact points of square
fn line_square(
    lx1: f32,
    ly1: f32,
    lx2: f32,
    ly2: f32,
    sx: f32,
    sy: f32,
    sw: f32,
    sh: f32,
) -> Vec<(f32, f32)> {
    let mut contacts: Vec<(f32, f32)> = Vec::new();

    // Left square edge
    let res = line_line(lx1, ly1, lx2, ly2, sx, sy, sx, sy + sh);
    if res.is_some() {
        contacts.push(res.unwrap());
    }

    // Right square edge
    let res = line_line(lx1, ly1, lx2, ly2, sx + sw, sy, sx + sw, sy + sh);
    if res.is_some() {
        contacts.push(res.unwrap());
    }

    // Bottom square edge
    let res = line_line(lx1, ly1, lx2, ly2, sx, sy, sx + sw, sy);
    if res.is_some() {
        contacts.push(res.unwrap());
    }

    // Top square edge
    let res = line_line(lx1, ly1, lx2, ly2, sx, sy + sh, sx + sw, sy + sh);
    if res.is_some() {
        contacts.push(res.unwrap());
    }

    contacts
}

fn line_line(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
) -> Option<(f32, f32)> {
    // calculate the direction of the lines
    let u_a = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3))
        / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));
    let u_b = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3))
        / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));

    if u_a >= 0f32 && u_a <= 1f32 && u_b >= 0f32 && u_b <= 1f32 {
        let intersection_x = x1 + (u_a * (x2 - x1));
        let intersection_y = y1 + (u_a * (y2 - y1));
        return Some((intersection_x, intersection_y));
    }

    None
}
