use minesweeper::app_parameters::*;

fn main() {
    let windows_info: WindowDescriptor = WindowDescriptor {
        width: 1000.0,
        height: 1000.0,
        title: "Minesweeper by vitos".to_owned(),
        resizable: true,
        ..Default::default()
    };

    App::new()
        .init_resource::<MSInfo>()
        .insert_resource(Msaa{ samples: 0 })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: windows_info,
            ..Default::default()
        })
        .set( ImagePlugin {
            default_sampler: ImageSampler::nearest_descriptor(),
        }))
        .add_system(close_on_esc)
        .add_startup_system(startup)
        .add_state(GameState::Intro)
        .add_system_set(SystemSet::on_update(GameState::Intro).with_system(intro))
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(init_ms))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(run_ms))
        .run();

}
