use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::AsBindGroup,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_systems(Startup, startup)
        .add_systems(Update, test)
        .run();
}

fn test(mut materials: ResMut<Assets<CustomMaterial>>) {
    for (hid, material) in materials.iter_mut() {
        material.color.set_r((material.color.r() + 0.01) % 1.0);
    }
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "test/shader_test.wgsl".into()
    }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let font = asset_server.load("test/FiraCode-Regular.ttf");
    let image = asset_server.load("test/image.png");
    let shader = asset_server.load("test/shader.wgsl");
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(2.5, 0.5, 0.0),
        material: materials.add(CustomMaterial { color: Color::BLUE }),
        ..default()
    });
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
            parent.spawn(ShaderNodeBundle {
                style: Style {
                    width: Val::Px(250.0),
                    height: Val::Px(250.0),
                    ..default()
                },
                shader: shader.into(),
                ..default()
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
