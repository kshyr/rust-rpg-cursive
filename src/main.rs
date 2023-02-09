use cursive::theme::Style;
use cursive::view::Margins;
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, ListView, MenuPopup, Menubar, Panel, SelectView,
    SliderView, TextView, ViewRef,
};
use cursive::{align, event::Key, Cursive, CursiveExt};
use cursive::{direction, traits::*};
use rand::Rng;

struct PlayerLoc {
    x: i32,
    y: i32,
    floor: i32,
}

#[derive(Clone)]
struct GameMap {
    tiles: [[char; 24 * 2 + 1]; 24],
}

struct UserData {
    player_loc: PlayerLoc,
    map: GameMap,
    //    map: Vec<Vec<String>>,
}

fn main() {
    let mut c = cursive::default();

    let mut runner = c.runner();

    runner.load_toml(include_str!("./theme.toml")).unwrap();

    runner.add_global_callback('q', |s| s.quit());

    // Start menu
    runner.add_layer(cursive::views::Dialog::around(
        cursive::views::LinearLayout::vertical()
            .child(cursive::views::Button::new("New game", move |s| {
                draw_map(s);
            }))
            .child(cursive::views::Button::new("Settings", |s| {}))
            .child(cursive::views::Button::new("Quit", |s| s.quit())),
    ));

    runner.add_global_callback('t', |s| {
        s.add_layer(
            Dialog::around(EditView::new().on_submit(showpopup).with_name("name"))
                .title("Enter a new name: ")
                .button("OK", |s| {
                    let name = s
                        .call_on_name("name", |view: &mut EditView| view.get_content())
                        .unwrap();
                    showpopup(s, &name);
                })
                .dismiss_button("Cancel"),
        );
    });

    runner
        .menubar()
        .add_subtree(
            "File",
            cursive::menu::Tree::new()
                .leaf("Quit", |s| s.quit())
                .leaf("Open", |s| {
                    s.add_layer(Dialog::info("Open file"));
                }),
        )
        .add_subtree(
            "Edit",
            cursive::menu::Tree::new()
                .leaf("Copy", |s| {
                    s.add_layer(Dialog::info("Copy"));
                })
                .leaf("Paste", |s| {
                    s.add_layer(Dialog::info("Paste"));
                }),
        );

    runner.add_global_callback(Key::Esc, |s| s.select_menubar());

    runner.set_user_data(UserData {
        player_loc: PlayerLoc {
            x: 1,
            y: 1,
            floor: 9,
        },
        map: GameMap {
            tiles: [[' '; 24 * 2 + 1]; 24],
        },
    });

    while runner.is_running() {
        runner.refresh();

        runner.step();
    }
}

fn showpopup(s: &mut Cursive, name: &str) {
    s.pop_layer();
    s.add_layer(Dialog::info(format!("Hello {}!", name)));
}

fn draw_map(s: &mut Cursive) {
    let height = 24;
    let width = 24;
    let mut player_x = s.user_data::<UserData>().unwrap().player_loc.x;
    let mut player_y = s.user_data::<UserData>().unwrap().player_loc.y;
    s.pop_layer();
    s.add_layer(
        LinearLayout::horizontal()
            .child(
                LinearLayout::vertical()
                    .child(Dialog::around(TextView::new("Floor: 9").with_name("floor")))
                    .child(Dialog::around(TextView::new("map").with_name("map"))),
            )
            .child(Dialog::around(
                LinearLayout::vertical()
                    .child(SelectView::new().item("Item 1", 1).item("Item 2", 2))
                    .child(SelectView::new().item("Item 1", 1).item("Item 2", 2)),
            )),
    );

    // s.find_name::<TextView>("map")
    //     .unwrap()
    //     .set_content(construct_map(s, height, width, player_x, player_y));

    s.add_global_callback('w', move |s| move_player(s, height, width, "north"));
    s.add_global_callback('a', move |s| move_player(s, height, width, "west"));
    s.add_global_callback('s', move |s| move_player(s, height, width, "south"));
    s.add_global_callback('d', move |s| move_player(s, height, width, "east"));
}

fn construct_map(s: &mut Cursive, height: i32, width: i32, player_x: i32, player_y: i32) -> String {
    for i in 0..height {
        for j in 0..width * 2 + 1 {
            let x: char;
            let current_tile = s.user_data::<UserData>().unwrap().map.tiles[i as usize][j as usize];
            let y_axis_stairs_up_placement = 11..13;
            let y_axis_stairs_down_placement = 14..16;
            let x_axis_stairs_placement = 8..10;

            if i == player_y && j == player_x * 2 {
                x = '*'; // player
            } else if y_axis_stairs_down_placement.contains(&i)
                && (x_axis_stairs_placement.contains(&j))
            {
                x = '>';
            } else if y_axis_stairs_down_placement.contains(&i)
                && (x_axis_stairs_placement.contains(&(j - 1)))
            {
                x = 'v';
            } else if y_axis_stairs_up_placement.contains(&i)
                && (x_axis_stairs_placement.contains(&j))
            {
                x = '>';
            } else if y_axis_stairs_up_placement.contains(&i)
                && (x_axis_stairs_placement.contains(&(j - 1)))
            {
                x = '^';
            } else if i == 0
                || (i == 4 && (j >= 8 && j <= width * 2 - 8))
                || (i == height - 5 && (j >= 8 && j <= width * 2 - 8))
                || i == height - 1
                || j == 0
                || (j == 8 && (i >= 4 && i <= height - 5))
                || (j == width * 2 - 8 && (i >= 4 && i <= height - 5))
                || j == width * 2
                || (x_axis_stairs_placement.contains(&(j - 1))
                    && (y_axis_stairs_down_placement.contains(&(i - 1))
                        || y_axis_stairs_down_placement.contains(&(i + 1))
                        || y_axis_stairs_up_placement.contains(&(i + 1))))
            {
                x = '#'; // walls
            } else if current_tile == ' ' && current_tile == '*' {
                x = current_tile;
            } else {
                x = ' '; // empty space
            }

            s.user_data::<UserData>().unwrap().map.tiles[i as usize][j as usize] = x;
        }
    }
    let map = s.user_data::<UserData>().unwrap().map.tiles;
    let mut map_string = String::new();
    for rows in map.iter() {
        for cols in rows.iter() {
            map_string.push_str(cols.to_string().as_str());
        }
        map_string.push_str("\n");
    }

    map_string
}

fn move_player(s: &mut Cursive, height: i32, width: i32, direction: &str) {
    let player_x = s.user_data::<UserData>().unwrap().player_loc.x;
    let player_y = s.user_data::<UserData>().unwrap().player_loc.y;
    let tiles = s.user_data::<UserData>().unwrap().map.tiles;
    match direction {
        "north" => {
            if player_y > 0 && tiles[player_y as usize - 1][player_x as usize * 2] != '#' {
                s.user_data::<UserData>().unwrap().player_loc.y -= 1
            };
        }
        "south" => {
            if player_y < height - 1 && tiles[player_y as usize + 1][player_x as usize * 2] != '#' {
                s.user_data::<UserData>().unwrap().player_loc.y += 1
            };
        }
        "east" => {
            if player_x < width - 1 && tiles[player_y as usize][player_x as usize * 2 + 2] != '#' {
                s.user_data::<UserData>().unwrap().player_loc.x += 1
            };
        }
        "west" => {
            if player_x > 0 && tiles[player_y as usize][player_x as usize * 2 - 2] != '#' {
                s.user_data::<UserData>().unwrap().player_loc.x -= 1
            };
        }
        _ => {}
    }
    let player_x = s.user_data::<UserData>().unwrap().player_loc.x;
    let player_y = s.user_data::<UserData>().unwrap().player_loc.y;

    let y_axis_stairs_up_placement = 11..13;
    let y_axis_stairs_down_placement = 14..16;
    let x_axis_stairs_placement = 8..10;

    if player_x == 5 && y_axis_stairs_up_placement.contains(&player_y) {
        s.user_data::<UserData>().unwrap().player_loc.x = 3;
        s.user_data::<UserData>().unwrap().player_loc.y = 14;
        s.user_data::<UserData>().unwrap().player_loc.floor += 1;
        floor_gen(s);
    } else if player_x == 5 && y_axis_stairs_down_placement.contains(&player_y) {
        s.user_data::<UserData>().unwrap().player_loc.x = 3;
        s.user_data::<UserData>().unwrap().player_loc.y = 11;
        s.user_data::<UserData>().unwrap().player_loc.floor -= 1;
        floor_gen(s);
    } else if x_axis_stairs_placement.contains(&player_x) && player_y == 11 {
        s.user_data::<UserData>().unwrap().player_loc.x = 3;
        s.user_data::<UserData>().unwrap().player_loc.y = 14;
        s.user_data::<UserData>().unwrap().player_loc.floor += 1;
        floor_gen(s);
    } else if x_axis_stairs_placement.contains(&player_x) && player_y == 14 {
        s.user_data::<UserData>().unwrap().player_loc.x = 3;
        s.user_data::<UserData>().unwrap().player_loc.y = 11;
        s.user_data::<UserData>().unwrap().player_loc.floor -= 1;
        floor_gen(s);
    }

    let player_x = s.user_data::<UserData>().unwrap().player_loc.x;
    let player_y = s.user_data::<UserData>().unwrap().player_loc.y;

    s.find_name::<TextView>("map")
        .unwrap()
        .set_content(construct_map(s, height, width, player_x, player_y));
    s.find_name::<TextView>("floor")
        .unwrap()
        .set_content(format!(
            "Floor: {}",
            s.user_data::<UserData>().unwrap().player_loc.floor
        ));
}

fn floor_gen(s: &mut Cursive) {
    let rng = rand::thread_rng().gen_range(0..100);
    let tiles = s.user_data::<UserData>().unwrap().map.tiles;

    if rng < 50 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..tiles.len());
        let y = rng.gen_range(0..tiles[x].len());
        if tiles[x][y] == ' ' {
            s.user_data::<UserData>().unwrap().map.tiles[x][y] = 'M';
        }
    }
}
