use bevy::prelude::*;
use bevy::xr::XrTrackingSource;

pub struct XrLeftHand(Entity);
pub struct XrRightHand(Entity);

#[derive(Component)]
pub enum XrHandSide {
    Left,
    Right,
}

///Spawns hands
fn init_hands(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //TODO: Make hand mesh configurable
    //TODO: Consider spawning hands as poses become available
    let left_hand = commands
        .spawn()
        .insert(XrHandSide::Left)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.0, 0.0),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .id();
    commands.insert_resource(XrLeftHand(left_hand));

    let right_hand = commands
        .spawn()
        .insert(XrHandSide::Right)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 0.0, 0.8),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .id();
    commands.insert_resource(XrRightHand(right_hand));
}

///Sets hand transforms to corresponding `XrPose` transforms
fn update_hand_transforms(
    mut hand_query: Query<(&XrHandSide, &mut Transform)>,
    tracking_source: Res<XrTrackingSource>,
) {
    let [left_pose, right_pose] = tracking_source.hands_pose();
    for (hand_side, mut hand_transform) in hand_query.iter_mut() {
        let pose = match hand_side {
            XrHandSide::Left => &left_pose,
            XrHandSide::Right => &right_pose,
        };

        let pose = match pose {
            Some(pose) => pose,
            None => continue,
        };

        //TODO: This should probably be replaced with a "should" position
        //so actual movements can be subject to bounds (such as collision)
        hand_transform.translation = pose.transform.position;
        hand_transform.rotation = pose.transform.orientation;
    }
}

///Provides hand entities which track the controllers
pub struct XrHandsPlugin;

impl Plugin for XrHandsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(XrLeftHand)
            .insert_resource(XrRightHand)
            .add_startup_system(init_hands)
            .add_system(update_hand_transforms);
    }
}

///Marks entities as Grabbable
///TODO: Extend with callbacks and parameters
///to customize how each entity responds to grab actions
#[derive(Component)]
pub struct Grabbable;
