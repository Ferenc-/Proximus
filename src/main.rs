use adw::{AppInfo, AppWindow, AppWindowType};
use adw::prelude::*;
use std::env;

#[derive(Default)]
struct MyApp {
    window: AppWindow,
}

fn main() {
    //if !env::var("XDG_SESSION_TYPE").is_ok() {
    //}

    let application = AppInfo::builder()
        .application_id("com.example.Proximus")
        .application_name("Proximus")
        .application_version("0.1")
        .build();

    application.connect_activate(|app| {
        let row = ActionRow::builder()
            .activatable(true)
            .title("Click me")
            .build();
        row.connect_activated(|_| {
            eprintln!("Clicked!");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(SelectionMode::None)
            .css_classes(vec![String::from("boxed-list")])
            .build();
        list.append(&row);

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(&HeaderBar::new());
        content.append(&list);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Proximus")
            .default_width(350)
            // add content to window
            .content(&content)
            .build();
        window.present();
    });

/*
    // Create a menu bar
    let menu_bar = window.add_child(MenuBar::builder().build());

    // Add menu options
    let list_menu = menu_bar.add_child(MenuItem::builder()
        .label(Some("List"))
        .build());

    let passport_menu = menu_bar.add_child(MenuItem::builder()
        .label(Some("e-Passport"))
        .build());

    // Connect the menu options to actions
    list_menu.connect_activate(move |_| {
        // Do something when "List" is clicked
        println!("List clicked");
    });

    passport_menu.connect_activate(move |_| {
        // Display a form with 3 text fields
        let form = Form::builder()
            .add_child(Label::builder().label(Some("First Name"))
            .build())
            .add_child(TextField::builder().placeholder(Some("First Name"))
            .build())
            .add_child(Label::builder().label(Some("Last Name"))
            .build())
            .add_child(TextField::builder().placeholder(Some("Last Name"))
            .build())
            .add_child(Label::builder().label(Some("Date of Birth"))
            .build())
            .add_child(TextField::builder().placeholder(Some("Date of Birth"))
            .build())
            .build();

        window.add_child(form);
        form.show_all();

        // Connect the form submit event
        form.connect_submit(move |_, _| {
            // Show a message dialog box displaying the entered data
            let (first_name, last_name, date_of_birth) = form.get_child_at_index(0).downcast_ref::<TextField>().map(|t| t.text()).unwrap_or_default();
            let dialog = MessageDialog::builder()
                .application_id("com.example.myapp")
                .application_name("My App")
                .application_version("1.6")
                .modal(true)
                .text_format("hsize=fill; weight=normal; xscale=normal; yscale=normal")
                .buttons(&[ButtonType::Ok, ButtonType::Cancel])
                .message(format!("First Name: {}\nLast Name: {}\nDate of Birth: {}", first_name, last_name, date_of_birth))
                .build();

            dialog.show_all();
        });
    });
*/
    application.run();
}
