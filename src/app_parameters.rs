pub use bevy::{window::close_on_esc, prelude::*, render::{
    render_resource::SamplerDescriptor, 
    texture::ImageSampler
}};

use bevy::{sprite::collide_aabb::{Collision, collide}};
use std::{path::Path, cmp::max, collections::HashMap};

use crate::minesweeper::*;

const INTRO_FONT_SIZE: f32 = 60.0;
const INPUT_TEXT_FONT_SIZE: f32 = 120.0;

const NORMAL_BUTTON: Color = Color::rgb(0.75, 0.75, 0.75);
const HOVERED_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
const PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Resource, Clone, Copy, Default)]
pub struct MSInfo {
    width: usize,
    height: usize,
    bombs: usize
}

#[derive(Component)]
pub struct MS;

#[derive(Component)]
pub struct InputText;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Intro,
    Playing,
    Endgame
}

#[derive(Resource)]
pub struct GameRes {
    font: Handle<Font>,
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
    names.extend(vec!["bomb".to_owned(), "cell".to_owned(), "flag".to_owned()]);

    let mut imgs = HashMap::new();
    for e in names {
        imgs.insert(e.clone(), a.load(Path::new("img").join(e + ".png")));
    }

    c.insert_resource(GameRes {
        font: a.load(Path::new("fonts").join("Nunito-Regular.ttf")),
        imgs
    });
}

pub fn init(
    gr: Res<GameRes>,
    mut c: Commands,
) {
    // spawn starting text
    c.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Size of Minesweeper grid:".to_owned(),
                style: TextStyle {
                    font: gr.font.clone(),
                    font_size: INTRO_FONT_SIZE,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Left,
            },
        },
        transform: Transform {
            translation: Vec3 {
                x: -225.,
                y: 225.,
                z: 1.,
            },
            ..Default::default()
        },
        ..Default::default()
    });

    // spawn input text 
    c.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: "10".to_owned(),
                style: TextStyle {
                    font: gr.font.clone(),
                    font_size: INPUT_TEXT_FONT_SIZE,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Left,
            },
        },
        transform: Transform {
            translation: Vec3 {
                x: -225.,
                y: 175.,
                z: 1.,
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(InputText);

    // spawn start button
    c.spawn(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(450.), Val::Px(100.)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: BackgroundColor::from(NORMAL_BUTTON),
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: 0.,
                z: 1.
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Start!",
            TextStyle {
                font: gr.font.clone(),
                font_size: 80.,
                color: Color::rgb(1., 1., 1.)
            }
        ));
    });
}

pub fn init_ms(
    a: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
    mut c: Commands, 
    // mut cursor_moved_event_reader: EventReader<CursorMoved>,
    // mut cursor_position: Local<Vec2>,
    mut char_evr: EventReader<ReceivedCharacter>,
    mut ms_info: ResMut<MSInfo>,
    mut state: ResMut<State<GameState>>,
    mut chosen: Local<bool>,
    mut pressed: Local<bool>,
    mut text_query: Query<&mut Text, With<InputText>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    button_entity_query: Query<Entity, With<Button>>,
    button_text_entity_query: Query<Entity, With<Text>>,
) {
    let mut input_text;

    if !*chosen {
        for mut text in &mut text_query {
            input_text = &mut (*text).sections[0].value;

            for ev in char_evr.iter() {
                if '0' <= ev.char && ev.char <= '9' { 
                    input_text.push(ev.char);
                }
            }

            for (interaction, mut color) in &mut interaction_query {
                match *interaction {
                    Interaction::Clicked => {
                        *color = PRESSED_BUTTON.into();
                        *pressed = true;
                    }
                    Interaction::Hovered => {
                        *color = HOVERED_BUTTON.into();
                    }
                    Interaction::None => {
                        *color = NORMAL_BUTTON.into();
                    }
                }
            }

            if keys.just_pressed(KeyCode::Back) {
                if input_text.len() > 0 {
                    input_text.pop();
                }
            } else if keys.just_pressed(KeyCode::Return) || *pressed { // TODO add if statement for pressing button
                // change state and 
                match (*input_text).trim().parse::<isize>() {
                    Ok(ms_size) => {
                        if ms_size > 0 {
                            input_text.clear();
                            // change info of ms_info
                            *ms_info = MSInfo {
                                width: ms_size as usize,
                                height: ms_size as usize,
                                bombs: (ms_size * ms_size / 10) as usize,
                            };
                            *chosen = true;
                    }
                    },
                    _ => {
                        println!("Wrong input!"); // TODO make err graphical
                    }
                }
            }
        }
    }

    if *chosen {
        for e in button_text_entity_query.iter() {
            c.entity(e).despawn();
        }
        for e in button_entity_query.iter() {
            c.entity(e).despawn();
        }

        c.insert_resource(*ms_info);

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

        state.overwrite_set(GameState::Playing).unwrap();
    }
}

pub fn run_ms(
    // time: Res<Time>,
    gr: Res<GameRes>,
    ms_info: Res<MSInfo>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
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

        let window = windows.get_primary().unwrap();
        let grid_max = max(ms.width, ms.height) as f32;
        // let grid_min = min(ms.width, ms.height) as f32;
        let wind_min = f32::min(window.width(), window.height());
        
        let size = wind_min / (grid_max + 1.);
        let size_vec = Some(Vec2::new(
            size,
            size
        ));

        let pad_x = size/2.;
        let pad_y = size/2.;

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
                if collision == Collision::Inside {
                    if left_click {
                        ms.open(x, y);
                    } else if right_click {
                        ms.flag(x, y);
                        if ms.grid[y][x].flag {
                            *i = gr.imgs.get("flag").unwrap().clone();
                        } else {
                            *i = gr.imgs.get("cell").unwrap().clone();
                        }
                    } else {
                        s.color = Color::rgb(0.8, 0.8, 0.8)
                    }
                }
            } else {
                s.color = Color::rgb(1.0, 1.0, 1.0)
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

pub fn endgame_init(
    gr: Res<GameRes>,
    mut c: Commands,
) {    
    c.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Game Over!".to_owned(),
                style: TextStyle {
                    font: gr.font.clone(),
                    font_size: INTRO_FONT_SIZE,
                    color: Color::rgb(1.0, 0.1, 0.1),
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Center,
            },
        },
        transform: Transform {
            translation: Vec3 {
                z: 1.,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    
    c.spawn(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(400.), Val::Px(100.)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        background_color: BackgroundColor::from(Color::rgb(0.6, 0.6, 0.6)),
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: 0.,
                z: 1.
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Restart",
            TextStyle {
                font: gr.font.clone(),
                font_size: 80.,
                color: Color::rgb(1., 1., 1.)
            }
        ));
    });
}

pub fn endgame(
    windows: Res<Windows>,
    // mut interaction_query: Query<
    //     (&Interaction, &mut BackgroundColor, &Children),
    //     (Changed<Interaction>, With<Button>),
    // >,
    mut text_query: Query<(&mut Text, &mut Transform), Without<Button>>,
) {
    let window = windows.get_primary().unwrap();
    let (w, h) = (window.width(), window.height());

    for (mut _game_over_text, mut game_over_transform) in text_query.iter_mut() {
        *game_over_transform = Transform {
            translation: Vec3 {
                x: 0.,
                y: h/2. - 100.,
                z: 1.
            },
            ..Default::default()
        }
    }


}