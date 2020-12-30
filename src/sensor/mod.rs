use bevy::math::Vec2;

mod collision_system;
mod render_system;
pub mod sensor_plugin;
mod update_coordinates_system;

pub const SENSOR_REACH: f32 = 90.0;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SensorDetectedEnum {
    NOTHING,
    FOOD,
    FLOOR,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sensor {
    /// End point of sensor on map
    pub end_point: Vec2,
    /// End point of sensor from observer_point
    pub relative_end_point: Vec2,
    // Distance from observer_point to end_point
    pub distance: f32,
    // What was detected by sensor
    pub detected: SensorDetectedEnum,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SensorSet {
    /// Sensors
    pub sensors: Vec<Sensor>,
    /// Range of view in rad from -range to +range
    pub range: f32,
    /// Observer point of sensorset pointing to center
    pub observer_point: Vec2,
}


impl Default for Sensor {
    fn default() -> Sensor {
        Sensor {
            end_point: Vec2::default(),
            relative_end_point: Vec2::default(),
            distance: SENSOR_REACH,
            detected: SensorDetectedEnum::NOTHING,
        }
    }
}

impl SensorSet {
    pub fn new(_count: i32, _range: f32) -> SensorSet {
        SensorSet {
            sensors: (0.._count).map(|_| Sensor::default()).collect(),
            range: _range,
            observer_point: Vec2::zero(),
        }
    }
}
