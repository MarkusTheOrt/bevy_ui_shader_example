use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef}, ui::{UiMaterial, MaterialNodeBundle, UiMaterialPlugin},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiMaterialPlugin::<CustomUiMaterial>::default())
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f9c447de-9875-4962-ae25-b34b286ecc0b"]
pub struct CustomUiMaterial {
    #[uniform(0)]
    pub percent: f32,
}

impl UiMaterial for CustomUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "test/shader.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "test/shader.wgsl".into()
    }
}

fn update(
    time: Res<Time>,
    mut mats: ResMut<Assets<CustomUiMaterial>>,
    query: Query<&Test>
) {

    let asset = query.single();
    let asset_t = mats.get_mut(&asset.handle);
    if let Some(mat) = asset_t {
        mat.percent = mat.percent + (time.delta_seconds() / 1.0);
        if mat.percent > 1.0 {
            mat.percent = mat.percent - 1.0;
        }
    }

}

#[derive(Component)]
pub struct Test {
    handle: Handle<CustomUiMaterial>
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ui_mats: ResMut<Assets<CustomUiMaterial>>
) {
    let font = asset_server.load("test/FiraCode-Regular.ttf");
    let image = asset_server.load("test/image.png");
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    let handle = ui_mats.add(CustomUiMaterial { percent: 0.5 });
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(250.0),
                    height: Val::Px(250.0),
                    ..default()
                },
                image: UiImage {
                    texture: image,
                    ..default()
                },
                ..default()
            });
            parent.spawn(MaterialNodeBundle {
                style: Style {
                    width: Val::Px(250.0),
                    height: Val::Px(250.0),
                    ..default()
                },
                material: handle.clone_weak(),
                ..default()
            }).insert(Test {
                handle,
                });
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Test Text",
                    TextStyle {
                        font,
                        font_size: 28.,
                        color: Color::BLACK,
                    },
                ),

                ..default()
            });
        });
}
