use yew::prelude::*;

#[function_component(Hello)]
pub fn hello() -> Html {
    html! {
        <div>
            <h1>{ "Hello, Sera" }</h1>
            <p>{ "Welcome to Yew with Salvo!" }</p>
        </div>
    }
}