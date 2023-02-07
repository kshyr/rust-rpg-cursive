use cursive::theme::Style;
use cursive::traits::*;
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, ListView, MenuPopup, Menubar, Panel, SliderView,
    TextView, ViewRef,
};
use cursive::{align, event::Key, Cursive, CursiveExt};
fn main() {
    let mut c = cursive::default();

    let mut runner = c.runner();
    runner.load_toml(include_str!("./theme.toml")).unwrap();

    runner.add_global_callback('q', |s| s.quit());

    // Start menu
    runner.add_layer(cursive::views::Dialog::around(
        cursive::views::LinearLayout::vertical()
            .child(cursive::views::Button::new("New game", |s| {}))
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
