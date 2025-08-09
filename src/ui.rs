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

// ---

fn startup (
    mut commands : Commands
) {
    commands.spawn(
        Node{
            width : Val::Vw(100.0),
            height: Val::Vh(100.0),
            justify_content: JustifyContent::SpaceBetween, 
            ..default()
        },
    )
    .with_children(|root_ui| {
        root_ui.spawn((
            Node {
                height: Val::Percent(100.),
                border: UiRect::all(Val::Px(2.)),
                overflow: Overflow::clip_y(),
                padding: UiRect::all(Val::Px(10.)),
                flex_basis: Val::Percent(30.),
                ..default()
            },
            Visibility::Hidden,
            GameMenu,
        ))

        .with_children(|parent| {
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ScrollingList::default(),
            ))
            .with_children(|list_container| {
                for i in get_list() {
                    list_container.spawn(list_item(&i));
                }
            })
            ;
        })
        ;


        root_ui.spawn((
            Node {
                height: Val::Px(100.),
                border: UiRect::all(Val::Px(2.)),
                padding: UiRect::all(Val::Px(20.)),
                justify_content: JustifyContent::End,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                flex_basis: Val::Percent(70.),
                ..default()
            },
            BorderRadius::new(Val::Px(10.), Val::Px(10.), Val::Px(10.), Val::Px(10.)),
            Visibility::Hidden,
            GameHint,
            BackgroundColor(Color::srgba(0., 0., 0., 0.5))
        ))
        .with_child((
            Text::new("'e' : exit to menu , scroll - zoom, click on screen - set center ".to_string()),
            TextFont {font_size: 20., ..default()},
            TextColor(COLOR_BEIGE),                
        ))
        ;
   
    });
}

// ---

use std::fs::read_dir;
fn get_list() -> Vec<String> {
    read_dir("assets/patterns/")
    .unwrap()
    .filter_map(|f| f.ok())
    .map(|f| f.file_name().to_str().unwrap().to_string())
    .filter(|f| f.ends_with(".rle"))
    .collect()
}

// ---

fn list_item(text: &str) ->impl Bundle + use<> {
    (
        Button,
        Node {
            padding: UiRect::all(Val::Px(5.0)),
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Text::new(text.to_string()),
                TextColor(COLOR_BEIGE),
                TextFont {
                    font_size: 20.,
                    ..default()
                }                        
            )
        ]
    )
}

use bevy::input::mouse::MouseWheel;

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Node, &Transform, &Children)>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut node, transforn,  children) in &mut query_list {
            let items_height =  (children.len() * 20) as f32;
            let container_height = transforn.scale.y;
            let max_scroll = (items_height - container_height).max(0.);
            let dy = mouse_wheel_event.y * 40.;
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            node.top = Val::Px(scrolling_list.position);
        }
    }
}


fn interact_buttons(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut file_name : ResMut<CurrentFileName>,
    mut text_q: Query<(&Text, &mut TextColor)>
) {
    for (interaction,  cc) in &mut interaction_query {
        let Ok((text, mut color)) = text_q.get_mut(cc[0]) else {
            continue;
        };
        match *interaction {
            Interaction::Pressed => {
                file_name.0 = text.0.clone();
                next_state.set(GameState::Simulate);
                
            }
            Interaction::Hovered => {
                color.0 =  COLOR_YELLOW;
            }
            Interaction::None => {
                color.0 =  COLOR_BEIGE;
            }
        }
    }
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

fn exit_setup(
    q: Single<&mut Visibility, With<GameMenu>>,
    q_s : Single<&mut Visibility, (With<GameHint>, Without<GameMenu>)>
) {
    *q.into_inner() = Visibility::Hidden;
    *q_s.into_inner() = Visibility::Visible;
}

// ---

fn enter_setup(
    q: Single<&mut Visibility, With<GameMenu>>,
    q_s : Single<&mut Visibility, (With<GameHint>, Without<GameMenu>)>
) {
    *q.into_inner() = Visibility::Visible;
    *q_s.into_inner() = Visibility::Hidden
}
