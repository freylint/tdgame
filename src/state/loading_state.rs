//! Loading screen implementation module

use bevy::prelude::*;

use crate::ecs::Rotating;
use crate::state::{AppStates, STAGE_LOADING};

/// Loading state logic
struct LoadState;

/// Bevy plugin for LoadState
pub struct LoadStatePlugin;

/// Resources needed by LoadState
#[allow(unused)]
struct LoadStateResources {
    pub(crate) mat_icon: Handle<ColorMaterial>,
    pub(crate) mat_spinner: Handle<ColorMaterial>,
    pub(crate) fnt_fira_bold: Handle<Font>,
}

// Initialization logic block
impl LoadState {
    /// Creates entities for the LoadState
    fn spawn(com: &mut Commands, res: Res<'_, LoadStateResources>) {
        Self::spawn_sprite_progress_spinner(com, res.mat_spinner.clone());
        Self::spawn_text_loading(com, res.fnt_fira_bold.clone());
        Self::spawn_sprite_main(com, res.mat_icon.clone());
        Self::setup_world(com);
    }

    /// Spawns loading screen's camera
    fn setup_world(com: &mut Commands) {
        // Spawn 2D camera
        com.spawn(Camera2dBundle {
            /*
            camera: Default::default(),
            orthographic_projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
             */
            ..Default::default()
        });

        // Spawn UI camera
        com.spawn(CameraUiBundle {
            /*
            camera: Default::default(),
            orthographic_projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
             */
            ..Default::default()
        });
    }

    /// Spawns an ent w/ a sprite component in the center of the screen
    fn spawn_sprite_main(commands: &mut Commands, icon: Handle<ColorMaterial>) {
        // Create transform matrix
        let trans_mat =
            Mat4::from_scale_rotation_translation(Vec3::one(), Quat::identity(), Vec3::zero());

        // Spawn sprite using provided texture
        commands.spawn(SpriteBundle {
            material: icon,
            transform: Transform::from_matrix(trans_mat),
            ..Default::default()
        });
    }

    /// Spawns progress spinner ent
    fn spawn_sprite_progress_spinner(commands: &mut Commands, spinner: Handle<ColorMaterial>) {
        // Create transform matrix
        let trans_mat = Mat4::from_scale_rotation_translation(
            (0.1, 0.1, 1.0).into(),
            Quat::identity(),
            (0.0, -250.0, 0.0).into(),
        );

        // Spawn spinner
        commands
            .spawn(SpriteBundle {
                material: spinner,
                transform: Transform::from_matrix(trans_mat),
                ..Default::default()
            })
            .with(Rotating(-4.0));
    }

    /// Spawns loading text entity
    fn spawn_text_loading(commands: &mut Commands, font: Handle<Font>) {
        commands.spawn(TextBundle {
            style: Style {
                // Setup Margin
                size: Size::new(Val::Percent(40.0), Val::Percent(20.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(20.0),
                    top: Val::Percent(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                value: format!("TDGame version {}", env!("CARGO_PKG_VERSION")),
                font,
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::BLACK,
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                },
            },
            ..Default::default()
        });
    }
}

// Update logic block
impl LoadState {
    fn update(_com: &mut Commands, _res: Res<'_, LoadStateResources>) {
        // Do nothing
    }
}

// destruction logic block
impl LoadState {
    fn kill(_com: &mut Commands, _res: Res<'_, LoadStateResources>) {
        // Do nothing
    }
}

impl FromResources for LoadStateResources {
    fn from_resources(resources: &Resources) -> Self {
        // Get engine stores
        let mut res_colormat = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get_mut::<AssetServer>().unwrap();

        // Load assets
        Self {
            mat_icon: res_colormat.add(asset_server.load("tex/icon.png").into()),
            mat_spinner: res_colormat.add(asset_server.load("tex/spinner.png").into()),
            fnt_fira_bold: asset_server.load("fnt/FiraSans-Bold.ttf"),
        }
    }
}

impl Plugin for LoadStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<LoadStateResources>()
            .on_state_enter(STAGE_LOADING, AppStates::Load, LoadState::spawn.system())
            .on_state_update(STAGE_LOADING, AppStates::Load, LoadState::update.system())
            .on_state_exit(STAGE_LOADING, AppStates::Load, LoadState::kill.system());
    }
}
