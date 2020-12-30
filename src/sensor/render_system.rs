use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::shared::config::Config;

use super::{SensorDetectedEnum, SensorSet};

pub(crate) struct SensorRender;

pub(crate) fn update_sensor_render_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<Config>,
    query_sensor_render: Query<(Entity, &SensorRender)>,
    query_sensor: Query<(&Transform, &SensorSet)>,
) {

    if config.fish.render_sensor { 

        for (e, ss) in query_sensor_render.iter() {
            commands.despawn(e);
        }
    
        let blue = materials.add(Color::rgb(0.0, 0.0, 0.9).into());
        let red = materials.add(Color::rgb(0.9, 0.0, 0.0).into());
    
        for (t, ss) in query_sensor.iter() {
    
            for sensor in ss.sensors.iter() {
                let mut builder = PathBuilder::new();
                let mut sensor_color = blue.clone();

                builder.move_to(point(ss.observer_point.x(), ss.observer_point.y()));
                builder.line_to(point(sensor.end_point.x(), sensor.end_point.y()));
                builder.close();


                if sensor.detected == SensorDetectedEnum::FLOOR {
                    sensor_color = red.clone();


                    commands
                    .spawn(primitive(
                        red.clone(),
                        &mut meshes,
                        ShapeType::Circle(3.0),
                        TessellationMode::Fill(&FillOptions::default()),
                        Vec3::new(sensor.end_point.x(), sensor.end_point.y(), 0.0),
                    ))
                    .with(SensorRender)
                    ;                
    
                }

                let path = builder.build();

                commands
                .spawn(
                    path.stroke(
                        sensor_color,
                        &mut meshes,
                        Vec3::new(0.0, 0.0, 0.0),
                        &StrokeOptions::default()
                            .with_line_width(1.3)
                            .with_line_cap(LineCap::Square)
                            .with_line_join(LineJoin::Round),
                    ),
                )
                .with(SensorRender)
                ;                



            }
    
        }        

    }


}
