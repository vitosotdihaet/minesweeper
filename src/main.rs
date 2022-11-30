use minesweeper::app_parameters::*;
use minesweeper::minesweeper::*;

fn main() {
    let windows_info: WindowDescriptor = WindowDescriptor {
        width: 1000.0,
        height: 600.0,
        title: "Minesweeper by vitos".to_owned(),
        resizable: true,
        ..Default::default()
    };

    App::new()
        .insert_resource(windows_info)
        .insert_resource(Msaa{ samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system)
        .add_startup_system(startup)
        .add_state(GameState::Intro)
        .add_system_set(SystemSet::on_update(GameState::Intro).with_system(intro))
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(init_ms))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(run_ms))
        .run();

}
