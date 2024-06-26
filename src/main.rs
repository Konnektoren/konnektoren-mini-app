use konnektoren_mini_app::prelude::App;
use log::Level;

fn main() {
    console_log::init_with_level(Level::Trace).expect("error initializing log");
    log::info!("Starting app");
    yew::Renderer::<App>::new().render();
}
