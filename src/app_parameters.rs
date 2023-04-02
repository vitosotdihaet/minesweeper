pub use bevy::{
    prelude::*,
    render::{render_resource::SamplerDescriptor, texture::ImageSampler},
    window::close_on_esc,
};

use bevy::{
    sprite::{
        collide_aabb::{collide, Collision},
        Anchor,
    },
    window::PrimaryWindow,
};

use std::{cmp::max, collections::HashMap, path::Path};

use crate::minesweeper::*;

const INTRO_FONT_SIZE: f32 = 60.0;
const INPUT_TEXT_FONT_SIZE: f32 = 120.0;

const NORMAL_BUTTON: Color = Color::rgb(0.7, 0.7, 0.7);
const HOVERED_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
const PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Resource, Clone, Copy, Default, Debug)]
pub struct MSInfo {
    width: usize,
    height: usize,
    mines: usize,
}

#[derive(Resource)]
pub struct GameRes {
    font: Handle<Font>,
    imgs: HashMap<String, Handle<Image>>,
}

#[derive(Resource, Clone, Copy, Default, Debug)]
pub struct GameWon {
    value: bool, 
}

#[derive(Component)]
pub struct MS;

#[derive(Component)]
pub struct InputText;

#[derive(Clone, PartialEq, Eq, Debug, Hash, States, Default)]
pub enum GameState {
    #[default]
    Intro,
    Playing,
    Endgame,
}

pub fn startup(
    a: Res<AssetServer>,
    mut c: Commands,
) {
    c.spawn(Camera2dBundle::default());

    // Load all assets
    let mut names = vec![];
    for i in 0..=8 {
        names.push(i.to_string().to_owned());
    }
    names.extend(vec!["mine".to_owned(), "cell".to_owned(), "flag".to_owned()]);

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
            ..default()
        },
        transform: Transform {
            translation: Vec3 {
                x: -225.,
                y: 225.,
                z: 1.,
            },
            ..default()
        },
        text_anchor: Anchor::TopLeft,
        ..default()
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
            ..default()
        },
        transform: Transform {
            translation: Vec3 {
                x: -225.,
                y: 175.,
                z: 1.,
            },
            ..default()
        },
        text_anchor: Anchor::TopLeft,
        ..default()
    })
    .insert(InputText);

    // spawn start button
    c.spawn(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(450.), Val::Px(100.)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor::from(NORMAL_BUTTON),
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: 0.,
                z: 1.
            },
            ..default()
        },
        ..default()
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
    mut char_evr: EventReader<ReceivedCharacter>,
    mut ms_info: ResMut<MSInfo>,
    mut state: ResMut<NextState<GameState>>,
    mut chosen: Local<bool>,
    mut clicked: Local<bool>,
    mut pressed: Local<bool>,
    mut text_query: Query<&mut Text, With<InputText>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    button_entity_query: Query<Entity, With<Button>>,
    button_text_entity_query: Query<Entity, With<Text>>,
) {
    if !*chosen {
        for mut text in &mut text_query {
            let input_text = &mut text.sections[0].value;

            for ev in char_evr.iter() {
                if '0' <= ev.char && ev.char <= '9' { 
                    input_text.push(ev.char);
                }
            }

            for (interaction, mut color) in &mut interaction_query {
                match *interaction {
                    Interaction::Clicked => {
                        *color = PRESSED_BUTTON.into();
                        *clicked = true;
                    }
                    Interaction::Hovered => {
                        *color = HOVERED_BUTTON.into();
                        if *clicked {
                            *pressed = true;
                            *clicked = false;
                        }
                    }
                    Interaction::None => {
                        *color = NORMAL_BUTTON.into();
                    }
                }
            }

            if keys.just_pressed(KeyCode::Back) {
                if !input_text.is_empty() {
                    input_text.pop();
                }
            } else if keys.just_pressed(KeyCode::Return) || *pressed {
                *pressed = false;
                // handle typed number
                match (*input_text).trim().parse::<isize>() {
                    Ok(ms_size) => {
                        if ms_size > 1 {
                            input_text.clear();
                            // change info of ms_info
                            *ms_info = MSInfo {
                                width: ms_size as usize,
                                height: ms_size as usize,
                                mines: max(1, ms_size * ms_size / 10) as usize,
                            };
                            *chosen = true;
                        } else {
                            println!("Invalid number!"); // TODO make err graphical
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
        *chosen = false;
        *pressed = false;

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
                    ..default()
                },
                ..default()
            })
            .insert(MS);
        }

        state.set(GameState::Playing);
    }
}

pub fn run_ms(
    // time: Res<Time>,
    gr: Res<GameRes>,
    ms_info: Res<MSInfo>,
    mouse_button_input: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<NextState<GameState>>,
    mut game_won: ResMut<GameWon>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut ms: Local<Minesweeper>,
    mut second_frame: Local<bool>,
    mut cursor_position: Local<Vec2>,
    mut sprites: Query<(&mut Sprite, &mut Transform, &mut Handle<Image>), With<MS>>,
) {
    if !*second_frame {
        *ms = Minesweeper::new(ms_info.width, ms_info.height, ms_info.mines);
        *second_frame = true;
        for (mut s, mut _p, mut _i) in &mut sprites{
            s.color = Color::rgb(1., 1., 1.);
        }
        return;
    }

    let Ok(window) = window_query.get_single() else {
        return;
    };

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

    // main game loop
    for (ind, (mut s, mut t, mut i)) in (&mut sprites).into_iter().enumerate() {
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
            ..default()
        };
        *t = trans;

        let collision_trans = Transform {
            translation: Vec3::new(
                tx + window.width()/2.,
                ty + window.height()/2.,
                0.0
                ),
            ..default()
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
                    s.color = Color::rgb(0.8, 0.8, 0.8);
                }
            }
        } else {
            s.color = Color::rgb(1.0, 1.0, 1.0);
        }

        s.custom_size = size_vec;

        // change sprite
        if ms.grid[y][x].revealed {
            if ms.grid[y][x].mine {
                *i = gr.imgs.get("mine").unwrap().clone();
            } else {
                let surr = ms.grid[y][x].surrounds;
                *i = gr.imgs.get(&surr.to_string()).unwrap().clone();
            }
        }

        if !ms.playing {
            *second_frame = false;
            game_won.value = ms.won;
        }
    }

    if !*second_frame {
        if !game_won.value {
            for (ind, (mut _s, mut _p, mut i)) in (&mut sprites).into_iter().enumerate() {
                let x = ind % ms.width;
                let y = ind / ms.width;

                if ms.grid[y][x].mine {
                    *i = gr.imgs.get("mine").unwrap().clone();
                }
            }
        }
        state.set(GameState::Endgame);
    }
}

pub fn endgame_init(
    gr: Res<GameRes>,
    mut c: Commands,
    game_won: ResMut<GameWon>,
) {
    let mut win_text = "Game Over!";
    let mut text_color = Color::rgb(1.0, 0.1, 0.1);
    if game_won.value {
        win_text = "You Won!";
        text_color = Color::rgb(0.1, 1.0, 0.1);
    }

    c.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: win_text.to_owned(),
                style: TextStyle {
                    font: gr.font.clone(),
                    font_size: INTRO_FONT_SIZE,
                    color: text_color,
                },
            }],
            alignment: TextAlignment::Center,
            ..default()
        },
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            },
            ..default()
        },
        ..default()
    });
    
    c.spawn(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(450.), Val::Px(100.)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor::from(Color::rgb(0.6, 0.6, 0.6)),
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: 0.,
                z: 1.
            },
            ..default()
        },
        ..default()
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
    window_query: Query<&Window, With<PrimaryWindow>>,
    ms_info: Res<MSInfo>,
    mut c: Commands,
    mut state: ResMut<NextState<GameState>>,
    mut text_query: Query<(&Text, &mut Transform), Without<Button>>,
    mut clicked: Local<bool>,
    mut pressed: Local<bool>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut ms_query: Query<
        (Entity, &mut Sprite, &mut Transform),
        (With<MS>, Without<Button>, Without<Text>),
    >,
    button_entity_query: Query<Entity, With<Button>>,
    text_entity_query: Query<Entity, With<Text>>,
) {

    let Ok(window) = window_query.get_single() else {
        return;
    };

    let (w, h) = (window.width(), window.height());

    let grid_max = max(ms_info.width, ms_info.height) as f32;
    // let grid_min = min(ms.width, ms.height) as f32;
    let wind_min = f32::min(w, h);

    let size = wind_min / (grid_max + 1.);
    let size_vec = Some(Vec2::new(
        size,
        size
    ));

    let pad_x = size/2.;
    let pad_y = size/2.;

    for (ind, (_e, mut s, mut t)) in (&mut ms_query).into_iter().enumerate() {
        let x = ind % ms_info.width;
        let y = ind / ms_info.width;

        let tx = pad_x + (x as f32 - ms_info.width as f32  / 2.) * size;
        let ty = pad_y + (y as f32 - ms_info.height as f32 / 2.) * size;

        let trans = Transform {
            translation: Vec3::new(
                tx,
                ty,
                0.0
                ),
            ..default()
        };
        *t = trans;
        s.custom_size = size_vec;
    }

    // adjust position of "game over" text
    for (_game_over_text, mut game_over_transform) in text_query.iter_mut() {
        *game_over_transform = Transform {
            translation: Vec3 {
                x: 0.,
                y: h/2. - 100.,
                z: 1.
            },
            ..default()
        }
    }

    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                *clicked = true;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                if *clicked {
                    *pressed = true;
                    *clicked = false;
                }
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }

    if *pressed {
        *pressed = false;

        for (e, _s, _t) in ms_query.iter() {
            c.entity(e).despawn();
        }
        for e in text_entity_query.iter() {
            c.entity(e).despawn();
        }
        for e in button_entity_query.iter() {
            c.entity(e).despawn();
        }

        state.set(GameState::Intro);
    }
}
