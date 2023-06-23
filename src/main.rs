mod app;
mod calculate;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
