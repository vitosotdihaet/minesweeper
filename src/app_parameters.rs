pub use bevy::{input::system::exit_on_esc_system, prelude::*};
use std::{path::Path, cmp::min};

use crate::minesweeper::*;

const INTRO_FONT_SIZE: f32 = 60.0;

#[derive(Component)]
pub struct MS;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Intro,
    Playing
}

pub struct GameRes {
    font_m: Handle<Font>,
    // sprites: Vec<SpriteBundle>
}

pub fn startup(mut c: Commands, a: Res<AssetServer>) {
    c.spawn_bundle(UiCameraBundle::default());
    c.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    // let mut sp = vec![];
    // let mut names = vec![];
    // for i in 1..=8 {
    //     names.push(i.to_string());
    // }

    // names.extend(vec![
    //     "bomb.png".to_owned(),
    //     "cell.png".to_owned(),
    //     "flag.png".to_owned(),
    //     "over_cell.png".to_owned()
    // ]);

    // for e in names {
    //     sp.push(SpriteBundle {
    //         texture: a.load(Path::new("imgs").join(e)),
    //         ..Default::default()
    //     } );
    // }

    c.insert_resource(GameRes {
        font_m: a.load(Path::new("fonts").join("Nunito-Regular.ttf")),
        // sprites: sp
    });
}

pub fn intro(
    mut c: Commands,
    mut frame_count: Local<usize>,
    mut query: Query<(Entity, &mut Text)>,
    mut state: ResMut<State<GameState>>,
    gr: Res<GameRes>
    ) {
    *frame_count += 1;

    if *frame_count == 1 {
        c.spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Minesweeper!".to_owned(),
                    style: TextStyle {
                        font: gr.font_m.clone(),
                        font_size: INTRO_FONT_SIZE,
                        color: Color::BLACK,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            ..Default::default()
        });
    } else if *frame_count < 50 {
        let mut text: Mut<Text> = query.single_mut().1;
        text.sections[0].style.color = Color::rgba(1.0, 1.0, 1.0, (*frame_count as f32) / 150.0);
    } else if *frame_count == 50 {
        let e: Entity = query.single_mut().0;
        c.entity(e).despawn();
        state.overwrite_set(GameState::Playing).unwrap();
    }
}

pub fn init_ms(mut ms: Local<Minesweeper>) {
    *ms = Minesweeper::new(10, 10, 10);
}

pub fn run_ms(
    time: Res<Time>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut windows: ResMut<Windows>,
    mut cursor_moved_event_reader: EventReader<CursorMoved>,
    mut cursor_position: Local<Vec2>,
    mut ms: Local<Minesweeper>,
    mut sprites: Query<&mut Sprite, With<MS>>,
    mut c: Commands,
    a: Res<AssetServer>,
    gr: Res<GameRes>
    ) {
        let window = windows.get_primary_mut().unwrap();
        let grid_min = min(ms.width, ms.height) as f32;
        let wind_min = f32::min(window.width(), window.height());
        let size = Some(Vec2::new(wind_min / (grid_min + 1.), wind_min / (grid_min + 1.)));

        for y in 0..ms.height {
            for x in 0..ms.width {
                let pivot = Transform::from_translation(Vec3::new(
                    window.width()  / (grid_min + 1.) * (x as f32 + 1.) - window.width() /2.,
                    window.height() / (grid_min + 1.) * (y as f32 + 1.) - window.height()/2.,
                    0.,
                ));

                if ms.grid[y][x].flag {
                    c.spawn_bundle(SpriteBundle {
                        texture: a.load(Path::new("imgs").join("flag.png")),
                        transform: pivot,
                        sprite: Sprite { custom_size: size, ..Default::default() },
                        ..Default::default()
                    });
                } else if ms.grid[y][x].revealed {
                    if ms.grid[y][x].bomb {
                        c.spawn_bundle(SpriteBundle {
                            texture: a.load(Path::new("imgs").join("bomb.png")),
                            transform: pivot,
                            sprite: Sprite { custom_size: size, ..Default::default() },
                            ..Default::default()
                        });
                    } else {
                        if ms.grid[y][x].surrounds != 0 {
                            c.spawn_bundle(SpriteBundle {
                                texture: a.load(Path::new("imgs").join(ms.grid[y][x].surrounds.to_string() + ".png")),
                                transform: pivot,
                                sprite: Sprite { custom_size: size, ..Default::default() },
                                ..Default::default()
                            });
                        } else {
                            c.spawn_bundle(SpriteBundle {
                                texture: a.load(Path::new("imgs").join("over_cell.png")),
                                transform: pivot,
                                sprite: Sprite { custom_size: size, ..Default::default() },
                                ..Default::default()
                            });
                        }
                    }
                } else {
                    c.spawn_bundle(SpriteBundle {
                        texture: a.load(Path::new("imgs").join("cell.png")),
                        transform: pivot,
                        sprite: Sprite { custom_size: size, ..Default::default() },
                        ..Default::default()
                    });
                }

                
            }
        }
}
