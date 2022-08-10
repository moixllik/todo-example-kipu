use cursive::{
    traits::*,
    views::{Dialog, LinearLayout, EditView, Button, TextView, Checkbox,
    }, Cursive,
};

mod db;

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::new().title("TODO").button("Quit", |s| s.quit())
            .content(
                LinearLayout::vertical()
                    .child(
                        LinearLayout::horizontal()
                            .child(EditView::new().with_name("desc").fixed_width(30))
                            .child(Button::new("Add", |s| {
                                s.call_on_name("desc", |ev: &mut EditView| {
                                    let id = db::id();
                                    let desc = ev.get_content();

                                    ev.set_content("");
                                    db::post("data", &("todo/desc-".to_string() + &id), &desc);
                                });

                                refresh(s);
                            }))
                    )
                    .child(LinearLayout::vertical().with_name("active"))
                    .child(LinearLayout::vertical().with_name("completed"))
            )
    );

    refresh(&mut siv);

    siv.run();
}

fn refresh(s: &mut Cursive) {
    s.call_on_name("active", |elm: &mut LinearLayout| {
        elm.clear();
    });
    s.call_on_name("completed", |elm: &mut LinearLayout| {
        elm.clear();
    });
    let d = db::get("list", "todo/0/desc-").replace("/data/todo/desc-", "");
    let list = d.split("\n");

    for id in list {
        let ok = db::get("data", &("todo/ok-".to_owned() + id));
        let desc = db::get("data", &("todo/desc-".to_owned() + id));

        if ok == "" {
            add_task(s, "active", id.to_string(), desc, false);
        } else {
            add_task(s, "completed", id.to_string(), desc, true);
        }
    };
}

fn add_task(s: &mut Cursive, name: &str, id: String, desc: String, checked: bool) {
    let ok = id.clone();
    let mut layout = s.find_name::<LinearLayout>(name).unwrap();
    let btn = Button::new("Remove", move |s| {
        db::delete("data", &("todo/desc-".to_owned() + &id.clone()));
        db::delete("data", &("todo/ok-".to_owned() + &id.clone()));
        refresh(s);
    });
    let mut ch = Checkbox::new();
    ch.set_checked(checked);
    ch.set_on_change(move |s, _st| {
        db::post("data", &("todo/ok-".to_owned() + &ok), "1");
        refresh(s);
    });
    layout.add_child(LinearLayout::horizontal()
        .child(ch)
        .child(TextView::new(desc))
        .child(btn)
    );
}
