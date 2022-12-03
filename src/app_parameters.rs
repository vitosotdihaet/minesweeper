pub use bevy::{window::close_on_esc, prelude::*};

use bevy::sprite::collide_aabb::{Collision, collide};
use std::{path::Path, cmp::{min, max}, collections::HashMap};

use crate::minesweeper::*;

const INTRO_FONT_SIZE: f32 = 60.0;
// const IMGS_PATH: &Path = Path::new("imgs");

#[derive(Resource, Clone, Copy, Default)]
pub struct MSInfo {
    width: usize,
    height: usize,
    bombs: usize
}

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
    imgs: HashMap<String, Handle<Image>>
}

pub fn startup(
    mut c: Commands, a: Res<AssetServer>
) {
    c.spawn(Camera2dBundle::default());

    // Load all assets
    let mut names = vec!["open_cell".to_owned()];
    for i in 1..=8 {
        names.push(i.to_string().to_owned());
    }
    names.extend(vec!["bomb".to_owned(), "cell".to_owned(), "flag".to_owned(), "over_cell".to_owned()]);

    let mut imgs = HashMap::new();
    for e in names {
        imgs.insert(e.clone(), a.load(Path::new("img").join(e + ".png")));
    }

    c.insert_resource(GameRes {
        font_m: a.load(Path::new("fonts").join("Nunito-Regular.ttf")),
        imgs
    });
}

pub fn intro(
    mut c: Commands,
    gr: Res<GameRes>,
    mut state: ResMut<State<GameState>>,
    mut frame_count: Local<usize>,
    mut query: Query<(Entity, &mut Text)>,
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
            height: 10,
            bombs: 10
        };
        c.insert_resource(*ms_info);
    // }
    for _ in 0..ms_info.width * ms_info.height {
        c.spawn(SpriteBundle {
            texture: a.load(Path::new("img").join("cell.png")),
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
    gr: Res<GameRes>,
    ms_info: Res<MSInfo>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut windows: ResMut<Windows>,
    mut state: ResMut<State<GameState>>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut ms: Local<Minesweeper>,
    mut second_frame: Local<bool>,
    mut cursor_position: Local<Vec2>,
    mut sprites: Query<(&mut Sprite, &mut Transform, &mut Handle<Image>), With<MS>>,
) {
        if !*second_frame {
            *ms = Minesweeper::new(ms_info.width, ms_info.height, ms_info.bombs);
            *second_frame = true;
        }

        let window = windows.get_primary_mut().unwrap();
        let grid_max = max(ms.width, ms.height) as f32;
        // let grid_min = min(ms.width, ms.height) as f32;
        let wind_min = f32::min(window.width(), window.height());
        
        let size = wind_min / (grid_max + 1.);
        let size_vec = Some(Vec2::new(
            size,
            size
        ));

        let pad_x = (window.width() / ((ms.width as f32 + 1.)))/2.;
        let pad_y = (window.height() / ((ms.height as f32 + 1.)))/2.;

        if let Some(moved_cursor) = cursor_moved.iter().last() {
            *cursor_position = moved_cursor.position;    
        }

        let left_click = mouse_button_input.just_released(MouseButton::Left);
        let right_click = mouse_button_input.just_released(MouseButton::Right);
        
        let mx = cursor_position.x;
        let my = cursor_position.y;

        let mut ind = 0;
        for (mut s, mut p, mut i) in sprites.iter_mut() {
            // let mut last_i = &i;
            let x = ind % ms.width;
            let y = ind / ms.width;

            let tx = pad_x + (x as f32 - ms.width as f32  / 2.) * size;
            let ty = pad_y + (y as f32 - ms.height as f32 / 2.) * size;

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

            if let Some(collision) = collide(
                collision_trans.translation,
                size_vec.unwrap(),
                Vec3::new(mx, my, 0.),
                Vec2::new(1.0, 1.0)
            ) {
                // println!("damn... {:?} at {} {}", _c, x, y);
                if collision == Collision::Inside {
                    if left_click {
                        ms.open(x, y);
                    } else if right_click {
                        ms.flag(x, y);
                    } else {
                        *i = gr.imgs.get("over_cell").unwrap().clone();
                    }
                }
            } else {
                *i = gr.imgs.get("cell").unwrap().clone();
            }

            s.custom_size = size_vec;

            if ms.grid[y][x].flag {
                *i = gr.imgs.get("flag").unwrap().clone();
            } else if ms.grid[y][x].revealed {
                if ms.grid[y][x].bomb {
                    *i = gr.imgs.get("bomb").unwrap().clone();
                    state.overwrite_set(GameState::Endgame).unwrap();
                } else {
                    let surr = ms.grid[y][x].surrounds;
                    if surr == 0 {
                        *i = gr.imgs.get("open_cell").unwrap().clone();
                    } else {
                        *i = gr.imgs.get(&surr.to_string()).unwrap().clone();
                    }
                }
            }
            ind += 1;
        }
}

pub fn endgame(
    c: Commands,
) {

}