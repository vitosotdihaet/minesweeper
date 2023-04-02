use minesweeper::app_parameters::*;

fn main() {
    let primary_window: Option<Window> = Some(Window {
        resolution: (1000., 1000.).into(),
        title: "Minesweeper by vitos".to_owned(),
        resizable: true,
        ..default()
    });

    App::new()
        .init_resource::<MSInfo>()
        .init_resource::<GameWon>()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window,
            ..default()
        })
        .set(ImagePlugin {
            default_sampler: ImageSampler::nearest_descriptor(),
        }))
        .add_system(close_on_esc)
        .add_startup_system(startup)
        .add_state::<GameState>()
            .add_system(init.in_schedule(OnEnter(GameState::Intro)))
            .add_system(init_ms.in_set(OnUpdate(GameState::Intro)))
            .add_system(run_ms.in_set(OnUpdate(GameState::Playing)))
            .add_system(endgame_init.in_schedule(OnEnter(GameState::Endgame)))
            .add_system(endgame.in_set(OnUpdate(GameState::Endgame)))
        .run();
}
