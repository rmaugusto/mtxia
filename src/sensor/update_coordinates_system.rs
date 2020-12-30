use super::{SensorSet, SENSOR_REACH};
use bevy::prelude::*;

pub fn update_sensor_coordinates(mut q_sensorset: Query<(Entity, &Transform, &mut SensorSet)>) {
    for (_, t, mut ss) in q_sensorset.iter_mut() {
        let obs = t.translation.truncate();
        //Current angle of sensor
        let (axis, ang) = t.rotation.to_axis_angle();

        let ang = match axis.z() > 0.0 {
            true => -ang,
            false => ang,
        };

        //First angle to left
        let mut min_ang = ang - ss.range;
        //Last angle to right
        let max_ang = ang + ss.range;
        //Total angle sliced
        let ang_slice: f32 = match ss.sensors.len() {
            1 => ang,
            _ => (max_ang - min_ang) / (ss.sensors.len() - 1) as f32,
        };

        if ss.sensors.len() == 1 {
            min_ang = ang;
        }

        ss.observer_point = obs;

        for (iline, mut sensor) in ss.sensors.iter_mut().enumerate() {
            let current_ang = min_ang + (ang_slice * iline as f32);
            let sensor_reach = SENSOR_REACH;

            sensor.end_point = Vec2::new(
                obs.x() + (sensor_reach * current_ang.sin()),
                obs.y() + (sensor_reach * current_ang.cos()),
            );

            sensor.relative_end_point = Vec2::new(
                sensor_reach * current_ang.sin(),
                sensor_reach * current_ang.cos(),
            );
        }
    }
}
