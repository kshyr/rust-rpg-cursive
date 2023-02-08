use cursive::theme::Style;
use cursive::traits::*;
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, ListView, MenuPopup, Menubar, Panel, SliderView,
    TextView, ViewRef,
};
use cursive::{align, event::Key, Cursive, CursiveExt};
use rand::Rng;

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
    s.pop_layer();
    s.add_layer(Dialog::info(construct_map(height, width)));
    s.add_global_callback('f', move |s| {
        s.pop_layer();
        s.add_layer(Dialog::info(construct_map(height, width)));
        s.cb_sink().send(Box::new(Cursive::noop)).unwrap();
    })
}

fn construct_map(height: i32, width: i32) -> String {
    let mut map = String::new();
    for _ in 0..height {
        for _ in 0..width {
            let x: &str = if rand::random() { "*" } else { "_" };
            map.push_str(x);
        }
        map.push_str("\n");
    }
    map
}
