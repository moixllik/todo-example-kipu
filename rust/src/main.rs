use cursive::{
    views::{Dialog, Button, LinearLayout, EditView, TextView, Checkbox},
    traits::*,
    Cursive};

mod db;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut siv = cursive::default();
    let lm = LinearLayout::horizontal()
        .child(EditView::new().with_name("desc").fixed_width(30))
        .child(Button::new("Add", new_task)
    );
    let la = LinearLayout::vertical().with_name("active");
    let lc = LinearLayout::vertical().with_name("completed");
    let d = db::get("list", "todo/0/desc-").await.replace("/data/todo/desc-", "");
    let list = d.split("\n");

    siv.add_layer(
        Dialog::new().title("TODO").button("Quit", |s| s.quit())
            .content(LinearLayout::vertical()
                .child(lm).child(la).child(lc)
            )
    );

    for id in list {
        let ok = db::get("data", &("todo/ok-".to_owned() + id)).await;
        let desc = db::get("data", &("todo/desc-".to_owned() + id)).await;
        
        if ok == "" {
            add_task(&mut siv, "active", id, desc, false);
        } else {
            add_task(&mut siv, "completed", id, desc, true);
        }
    }

    siv.run();
    Ok(())
}

fn new_task(s: &mut Cursive) {
    let id = db::id();
    let desc = s.call_on_name("desc", |elem: &mut EditView|
        elem.get_content()
    ).unwrap().to_string();

    add_task(s, "active", &id, desc, false);
}

fn remove_task(s: &mut Cursive) {
}

fn on_change(s: &mut Cursive, checked: bool) {
}

fn add_task(s: &mut Cursive, name: &str, id: &str, desc: String, checked: bool) {
    s.call_on_name(name, |elem: &mut LinearLayout| {
        let button = Button::new("Remove", remove_task);
        let mut checkbox = Checkbox::new();
        checkbox.set_checked(checked);
        checkbox.set_on_change(on_change);
        elem.add_child(LinearLayout::horizontal()
            .child(checkbox)
            .child(TextView::new(desc))
            .child(button)
        )
    }).unwrap();
}