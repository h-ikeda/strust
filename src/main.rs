mod model;

type Float = f64;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{"Hello World"}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
