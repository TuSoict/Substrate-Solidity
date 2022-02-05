mod model;
mod view;

fn main() {
    model::user::init();
    model::student::init();
    view::user::menu();
}
