use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let test = "test";
    html! {<div>
        <h1>{ "Hello World" }</h1>
        <h1>{test}</h1>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
