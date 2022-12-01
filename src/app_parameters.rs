use bevy::sprite::collide_aabb::collide;
pub use bevy::{window::close_on_esc, prelude::*};
use std::{path::Path, cmp::{min, max}, f32::consts::PI};

use crate::minesweeper::*;

const INTRO_FONT_SIZE: f32 = 60.0;
// const IMGS_PATH: &Path = Path::new("imgs");

#[derive(Resource, Clone, Copy, Default)]
pub struct MSInfo {
    width: usize,
    height: usize,
    bombs: usize
}

// impl Default for MSInfo {
//     fn default() -> Self {
//         MSInfo { width: 10, height: 10, bombs: 10 }
//     }
// }

#[derive(Component)]
pub struct MS;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Intro,
    Playing,
    Endgame
}

#[derive(Resource)]
pub struct GameRes {
    font_m: Handle<Font>,
    // sprites: Vec<SpriteBundle>
}

pub fn startup(mut c: Commands, a: Res<AssetServer>) {
    c.spawn(Camera2dBundle::default());
    
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
        c.spawn(Text2dBundle {
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

pub fn init_ms(
    mut c: Commands, 
    // mut cursor_moved_event_reader: EventReader<CursorMoved>,
    // mut cursor_position: Local<Vec2>,
    mut ms_info: ResMut<MSInfo>,
    a: Res<AssetServer>
) {
    // let mut chosen: bool = true;
    // while !chosen {
        *ms_info = MSInfo {
            width: 15,
            height: 15,
            bombs: 10
        };
        c.insert_resource(*ms_info);
    // }
    for _ in 0..ms_info.width * ms_info.height {
        c.spawn(SpriteBundle {
            texture: a.load(Path::new("imgs").join("cell.png")),
            sprite: Sprite {
                color: Color::Rgba{red: 1., green: 1., blue: 1., alpha: 1.},
                custom_size: Some(Vec2::new(1., 1.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MS);
    }
}

pub fn run_ms(
    // time: Res<Time>,
    mut second_frame: Local<bool>,
    mouse_button_input: Res<Input<MouseButton>>,
    ms_info: Res<MSInfo>,
    mut windows: ResMut<Windows>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut cursor_position: Local<Vec2>,
    mut ms: Local<Minesweeper>,
    mut sprites: Query<(&mut Sprite, &mut Transform, &Handle<Image>), With<MS>>,
    ) {
        if !*second_frame {
            *ms = Minesweeper::new(ms_info.width, ms_info.height, ms_info.bombs);
            *second_frame = true;
        }

        let window = windows.get_primary_mut().unwrap();
        let grid_max = max(ms.width, ms.height) as f32;
        let grid_min = min(ms.width, ms.height) as f32;
        let wind_min = f32::min(window.width(), window.height());
        let size = wind_min / (grid_min + 1.);
        let size_vec = Some(Vec2::new(
            size,
            size
        ));

        let mut mx: f32 = -10000.;
        let mut my: f32 = -10000.;

        if let Some(moved_cursor) = cursor_moved.iter().last() {
            *cursor_position = moved_cursor.position;
        }
        if mouse_button_input.just_released(MouseButton::Left) {
            mx = cursor_position.x;
            my = cursor_position.y;
        }

        let mut ind = 0;
        for (mut s, mut p, mut i) in sprites.iter_mut() {
            let x = ind % ms.width;
            let y = ind / ms.width;

            let tx = size/2. + (x as f32 - ms.width as f32  / 2.) * size;
            let ty = size/2. + (y as f32 - ms.height as f32 / 2.) * size;

            let trans = Transform {
                translation: Vec3::new(
                    tx,
                    ty,
                    0.0
                    ),
                ..Default::default()
            };

            *p = trans;
            let collision_trans = Transform {
                translation: Vec3::new(
                    tx + window.width()/2.,
                    ty + window.height()/2.,
                    0.0
                    ),
                ..Default::default()
            };

            if let Some(_c) = collide(
                collision_trans.translation,
                size_vec.unwrap(),
                Vec3::new(mx, my, 0.),
                Vec2::new(1.0, 1.0)
            ) {
                let off_x = window.width() as f32 - size;
                let off_y = window.height() as f32 - size;

                let gx = ((mx + size/2.) / off_x * (ms.width as f32)) as isize - 1;
                let gy = ((my + size/2.) / off_y * (ms.height as f32)) as isize - 1;
                
                // println!("damn... {:?} at {} {}", _c, gx, gy);
                if (gx as usize) < ms.width && (gy as usize) < ms.height {
                    ms.open(gx as usize, gy as usize);
                }
            }

            s.custom_size = size_vec;

            if ms.grid[y][x].flag {
                s.color = Color::rgb(
                    1.0,
                    0.1,
                    0.1
                )
            } else if ms.grid[y][x].revealed {
                if ms.grid[y][x].bomb {
                    s.color = Color::rgb(
                        0.1,
                        0.1,
                        0.1
                    )
                } else {
                    let surr = ms.grid[y][x].surrounds; 
                    if surr != 0 {
                        s.color = Color::rgb(
                            (surr as f32 * PI / 8.).sin(),
                            0.3,
                            (surr as f32 * PI / 8.).cos()
                        )
                    } else {
                        s.color = Color::rgb(
                            0.3,
                            0.3,
                            0.3
                        )
                    }
                }
            } else {

            }
            ind += 1;
        }
}
