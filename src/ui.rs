use bevy::prelude::*;
use crate::GameState;

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

#[derive(Component)]
pub struct GameMenu;

#[derive(Component)]
pub struct GameHint;


#[derive(Resource, Default)]
pub struct CurrentFileName(pub String);

const COLOR_BEIGE: Color = Color::srgb(207., 185., 151.);
const COLOR_RED: Color = Color::srgb(1., 0., 0.);
const COLOR_YELLOW: Color = Color::srgb(1., 1., 0.);

pub struct UIPlugin;
impl  Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(OnEnter(GameState::Setup), enter_setup)
            .add_systems(Update, mouse_scroll.run_if(in_state(GameState::Setup)))
            .add_systems(Update, interact_buttons.run_if(in_state(GameState::Setup)))
            .add_systems(OnExit(GameState::Setup), exit_setup)
            .add_systems(Update, control.run_if(in_state(GameState::Simulate)))
        ;
    }
} 

fn startup (
    mut commands : Commands
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width : Val::Vw(100.0),
                height: Val::Vh(100.0),
                justify_content: JustifyContent::SpaceBetween, 
                ..default()
            },
            ..default() 
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    border: UiRect::all(Val::Px(2.)),
                    overflow: Overflow::clip_y(),
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            
            GameMenu
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style : Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },  
                    ..default()
                },
                ScrollingList::default(),
            ))
            .with_children(|parent| {
                for i in get_list() {
                    parent.spawn(
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(5.0)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::NONE.into(),
                            ..default()
                        }

                    ).with_children(|button| {
                        button.spawn(
                            TextBundle::from_section(
                                i.to_string(),
                                TextStyle {
                                    font_size: 20.,
                                    color: COLOR_BEIGE.into(),
                                    ..default()
                                }
                            ),
                        );
                    })
                    ;
                }
            })
            ;
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Px(60.),
                        width: Val::Percent(100.),
                        border: UiRect::all(Val::Px(2.)),
                        overflow: Overflow::clip_y(),
                        padding: UiRect::all(Val::Px(20.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::End,
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                },
         
                GameHint
            ))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "'e' : exit to menu , scroll - zoom, click on screen - set center ".to_string(),
                        TextStyle {
                            font_size: 20.,
                            color: COLOR_BEIGE.into(),
                            ..default()
                        },
                    )
                );
            });
        });
    
    });
}


use std::fs::read_dir;
fn get_list() -> Vec<String> {
    read_dir("assets/patterns/")
    .unwrap()
    .filter_map(|f| f.ok())
    .map(|f| f.file_name().to_str().unwrap().to_string())
    .filter(|f| f.ends_with(".rle"))
    .collect()
}

use bevy::input::mouse::MouseWheel;

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Children)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, children) in &mut query_list {

            let items_height =  (children.len() * 20) as f32;
            let container_height = query_node.get(parent.get()).unwrap().size().y;
            let max_scroll = (items_height - container_height).max(0.);
            let dy = mouse_wheel_event.y * 40.;
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}


fn interact_buttons(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    mut next_state: ResMut<NextState<GameState>>,
    mut file_name : ResMut<CurrentFileName>
) {
    for (interaction,  children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].style.color =  COLOR_RED.into();
                file_name.0 = text.sections[0].value.clone();
                next_state.set(GameState::Simulate);
                
            }
            Interaction::Hovered => {
                text.sections[0].style.color =  COLOR_YELLOW.into();
            }
            Interaction::None => {
                text.sections[0].style.color =  COLOR_BEIGE.into();
            }
        }
    }
}


fn exit_setup(
    mut q: Query<&mut Visibility, With<GameMenu>>,
    mut q_s : Query<&mut Visibility, (With<GameHint>, Without<GameMenu>)>
) {
    let mut v = q.get_single_mut().unwrap();
    *v = Visibility::Hidden;

    let mut v_s = q_s.get_single_mut().unwrap();
    *v_s = Visibility::Visible;

}

// ---

fn control(
    mut next_state: ResMut<NextState<GameState>>,
    keys : Res<ButtonInput<KeyCode>>
) {
    if keys.just_pressed(KeyCode::KeyE) {
        next_state.set(GameState::Setup);
    }
}

// ---

fn enter_setup(
    mut q: Query<&mut Visibility, With<GameMenu>>,
    mut q_s : Query<&mut Visibility, (With<GameHint>, Without<GameMenu>)>
) {
    let mut v = q.get_single_mut().unwrap();
    *v = Visibility::Visible;
    let mut v_s = q_s.get_single_mut().unwrap();
    *v_s = Visibility::Hidden;
}