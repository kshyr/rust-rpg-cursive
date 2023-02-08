use cursive::theme::Style;
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, ListView, MenuPopup, Menubar, Panel, SliderView,
    TextView, ViewRef,
};
use cursive::{align, event::Key, Cursive, CursiveExt};
use cursive::{direction, traits::*};
use rand::Rng;

struct PlayerLoc {
    x: i32,
    y: i32,
}

struct UserData {
    player_loc: PlayerLoc,
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
                new_game(s);
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
        player_loc: PlayerLoc { x: 0, y: 0 },
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

fn new_game(s: &mut Cursive) {
    s.pop_layer();

    s.add_layer(
        Dialog::around(
            ListView::new()
                .child(
                    "Heigth: ",
                    EditView::new().with_name("set height").fixed_width(3),
                )
                .child(
                    "Width",
                    EditView::new().with_name("set width").fixed_width(3),
                ),
        )
        .button("Start", |s| {
            let height = s
                .call_on_name("set height", |view: &mut EditView| view.get_content())
                .unwrap();
            let width = s
                .call_on_name("set width", |view: &mut EditView| view.get_content())
                .unwrap();
            draw_map(s, height.parse().unwrap(), width.parse().unwrap());
        }),
    );
}

fn draw_map(s: &mut Cursive, height: i32, width: i32) {
    let mut player_x = s.user_data::<UserData>().unwrap().player_loc.x;
    let mut player_y = s.user_data::<UserData>().unwrap().player_loc.y;
    s.pop_layer();
    s.add_layer(TextView::new(construct_map(height, width, player_x, player_y)).with_name("map"));
    s.add_global_callback('w', move |s| move_player(s, height, width, "north"));
    s.add_global_callback('a', move |s| move_player(s, height, width, "west"));
    s.add_global_callback('s', move |s| move_player(s, height, width, "south"));
    s.add_global_callback('d', move |s| move_player(s, height, width, "east"));
}

fn construct_map(height: i32, width: i32, player_x: i32, player_y: i32) -> String {
    let mut map = String::new();
    for i in 0..height {
        for j in 0..width * 2 {
            let x: &str = if i == player_y && j == player_x * 2 {
                "*"
            } else {
                " "
            };
            map.push_str(x);
        }
        map.push_str("\n");
    }
    map
}

fn move_player(s: &mut Cursive, height: i32, width: i32, direction: &str) {
    match direction {
        "north" => {
            if s.user_data::<UserData>().unwrap().player_loc.y > 0 {
                s.user_data::<UserData>().unwrap().player_loc.y -= 1
            };
        }
        "south" => {
            if s.user_data::<UserData>().unwrap().player_loc.y < height - 1 {
                s.user_data::<UserData>().unwrap().player_loc.y += 1
            };
        }
        "east" => {
            if s.user_data::<UserData>().unwrap().player_loc.x < width - 1 {
                s.user_data::<UserData>().unwrap().player_loc.x += 1
            };
        }
        "west" => {
            if s.user_data::<UserData>().unwrap().player_loc.x > 0 {
                s.user_data::<UserData>().unwrap().player_loc.x -= 1
            };
        }
        _ => {}
    }
    let player_x = s.user_data::<UserData>().unwrap().player_loc.x;
    let player_y = s.user_data::<UserData>().unwrap().player_loc.y;
    s.find_name::<TextView>("map")
        .unwrap()
        .set_content(construct_map(height, width, player_x, player_y));
}
