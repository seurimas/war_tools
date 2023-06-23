use app::WarModel;

mod app;
mod calculate;

fn main() {
    yew::Renderer::<WarModel>::new().render();
}
