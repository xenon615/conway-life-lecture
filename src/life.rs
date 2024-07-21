
use std::time::Duration; 
use bevy::{prelude::*, time::common_conditions::on_timer};
use std::fs;
use regex::{Captures, Regex};
use crate::{ui::CurrentFileName, GameState};

#[derive(Component)]
pub struct Index((usize, usize)); 
#[derive(Resource)]
pub struct Generation {
    pub state: Vec<Vec<(usize, usize)>> 
}
// #[derive(Debug)]
// pub struct SceneItem{
//     file: String,
//     shift_i: usize,
//     shift_j: usize
// } 

// #[derive(Debug, PartialEq, Eq)]
// pub struct ParseError;

// impl FromStr for SceneItem {
//     type Err = ParseError;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let ss = s.split_whitespace().map(|f| f.to_string()).collect::<Vec<String>>();
//         if ss.len() != 3 {
//             return Err(ParseError);
//         }
//         let file = &ss[0];
//         let Ok(i_shift) = ss[1].parse::<usize>() else {
//             return Err(ParseError);
//         };
//         let Ok(j_shift) = ss[2].parse::<usize>() else {
//             return Err(ParseError);
//         };

//         Ok(SceneItem {
//             file: file.clone(),
//             shift_i: i_shift,
//             shift_j: j_shift
//         })    
//     }

// }

// fn read_scene() -> impl Iterator<Item = SceneItem> 
// {
//     let contents = fs::read_to_string("assets/scene.txt").unwrap_or(String::new());
//     let mut t: Vec<SceneItem> = Vec::new();
//     if !contents.is_empty() {
//         for l in contents.lines() {
//             if l.starts_with("#") {
//                 continue;
//             }
//             let Ok(si) = SceneItem::from_str(l) else {
//                 continue;
//             };
//             t.push(si);                       
//         }
//     }
//     t.into_iter()
// }

fn parse_hr(contents: String) -> Vec<Vec<(usize, usize)>> {
    let mut tmp : Vec<Vec<(usize, usize)>> = Vec::new();
    for line in contents.lines() {
        if line.starts_with("!") {
            continue;
        }
        let ss = line.chars().map(|c| {
            let t = if c == '.' {0} else {1};
            (t, t)    
        }).collect();
        tmp.push(ss);                        
    }
    tmp
}

fn parse_rle(contents: String) -> Vec<Vec<(usize, usize)>> {
    let mut data = contents.lines().filter(|x| !x.starts_with('#')).collect::<Vec<&str>>();
    let params = data.remove(0);
    let re = Regex::new(r"(\d+)").unwrap();
    let xyr:Vec<usize> = re.find_iter(&params).map(|m| m.as_str().parse::<usize>().unwrap()).collect();
    let mut tmp = vec![vec![(0, 0); xyr[0]]; xyr[1]];
    let data_s = data.join("").replace('!', "");
    let re1 = Regex::new(r"(\d+)([\w\$])").unwrap();
    let replaced = re1.replace_all(&data_s, | caps: &Captures| {
        let n = &caps[1].parse::<usize>().unwrap();
        let s =  &caps[2].to_string();
        s.repeat(*n)
    });
    for (i, s) in replaced.split('$').enumerate() {
        for (j, c)  in s.chars().enumerate() {
            let u: usize = if c == 'b' {0} else {1};
            tmp[i][j] = (u, u);
        }
    }
    tmp
}

// ---

impl FromWorld for Generation {
    fn from_world(world: &mut World) -> Self {
        let field_dim = (100, 100);
        let mut field = vec![vec![(0,0); field_dim.1]; field_dim.0];
        let file_name  = world.resource::<CurrentFileName>();
        let contents = fs::read_to_string("assets/patterns/".to_string() + &file_name.0).unwrap_or(String::new());
        if !contents.is_empty() {
            let pattern = if file_name.0.ends_with(".rle") {parse_rle(contents)} else {parse_hr(contents)};
            let dim_i = pattern.len();
            let dim_j = pattern[0].len();
            if (dim_i < field_dim.0) && (dim_j < field_dim.1) {
                let shift_i = field_dim.0 / 2 -  dim_i / 2;
                let shift_j = field_dim.1 / 2 -  dim_j / 2;
                for (i, r) in pattern.iter().enumerate() {
                    for (j, c ) in r.iter().enumerate() {
                        field[i + shift_i][j + shift_j] = *c;
                    }
                }
            }
        }
        
        Generation{state: field}
    }
}


// ---

pub struct FieldPlugin;
impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentFileName::default());
        app.add_systems(OnEnter(GameState::Simulate), (init_data, init_field).chain());
        app.add_systems(Update, 
            calc
            .run_if(in_state(GameState::Simulate))
            .run_if(on_timer(Duration::from_millis(500)))
        );
    }
}

// ---

fn init_data (
    mut commands : Commands
) {
    commands.remove_resource::<Generation>();
    commands.init_resource::<Generation>();
}

// ---

fn init_field(
    mut gen : ResMut<Generation>,
    mut commands : Commands,
    mut meshes : ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    let count_i = gen.state.len();
    let count_j = gen.state[0].len();
    let size = 0.25;
    
    let start_i = (size * (count_i as f32 - 0.5)) * 0.5 + 20.;
    let start_j = (size * (count_j as f32 - 0.5)) * 0.5;

    let top_left = Vec3::new( -start_j, start_i, 0.);

    for (i, r)  in gen.state.iter_mut().enumerate() {
        for (j, c)  in r.iter_mut().enumerate() {
            let pos = Vec3::new(top_left.x + size * j as f32, top_left.y - size * i as f32, 0.);
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Plane3d::default().mesh().size(size, size)),
                    material : materials.add(
                        StandardMaterial {
                            emissive: Color::linear_rgb(0.1, 1., 0.2).into(),
                            ..default()
                        }
                    ),
                    transform: Transform::from_translation(pos).with_rotation(Quat::from_rotation_x((90_f32).to_radians())),
                    visibility: if c.0 > 0 {Visibility::Visible} else {Visibility::Hidden},       
                    // visibility: Visibility::Visible,
                    ..default()
                },
                Index((i, j))
            ));
        }
    }
}

fn calc (
    mut gen : ResMut<Generation>,
    mut cells_q: Query<(&mut Visibility, &Index)>,
) {
    if gen.state.is_empty() {
        return;
    }
    let last_i = (gen.state.len() -1)  as isize;
    let last_j = (gen.state[0].len() - 1) as isize;
    
    for (mut v, idx)  in  cells_q.iter_mut() {
        *v = if gen.state[idx.0.0][idx.0.1].1 > 0  {Visibility::Visible} else {Visibility::Hidden};
        
        let c_i = idx.0.0 as isize;
        let c_j = idx.0.1 as isize;
        
        let ns = [
            (
                if c_i == 0 {last_i} else {c_i - 1},
                if c_j == 0 {last_j} else {c_j - 1}
            ),
            (
                if c_i == 0 {last_i} else {c_i - 1},
                c_j
            ),
            (
                if c_i == 0 {last_i} else {c_i - 1},
                if c_j == last_j {0} else {c_j + 1}
            ),
            (
                c_i,
                if c_j == last_j {0} else {c_j + 1}
            ),
            (
                if c_i == last_i {0} else {c_i + 1},
                if c_j == last_j {0} else {c_j + 1}
            ),
            (
                if c_i == last_i {0} else {c_i + 1},
                c_j
            ),
            (
                if c_i == last_i {0} else {c_i + 1},
                if c_j == 0 {last_j} else {c_j - 1}
            ),
            (
                c_i,
                if c_j == 0 {last_j} else {c_j - 1}
            ),
        ];


        let count = ns.map(|(i, j)| gen.state[i as usize][j as usize].0).iter().filter(|h| **h > 0).count();
        
        if gen.state[idx.0.0][idx.0.1].0 == 0 {
            if count == 3 {
                gen.state[idx.0.0][idx.0.1].1 = 1;    
            }
        } else {
            if count != 2 &&  count != 3 {
                gen.state[idx.0.0][idx.0.1].1 = 0;                    
            }
        }   

        
    }

    for r  in gen.state.iter_mut() {
        for c in r.iter_mut() {
            c.0 = c.1;
        }
    }

}

