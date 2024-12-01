use yew::prelude::*;
use crate::components::hello::Hello;

#[function_component(Root)]
pub fn root() -> Html {
    html! {
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>{ "Hello, Sera" }</title>
            </head>
            <body>
                <div id="app">
                    <Hello />
                </div>
            </body>
        </html>
    }
}