use crate::Route;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[styled_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <div class={css!(r#"
            width: 100%;
            height: 75px;
            background-color: #202124;
            display: flex;
            position: sticky;
            justify-content: flex-start;
            align-items: center;

            a {
                color: white;
                height: 50px;
                margin: 0;
                padding: 0px 30px;
                display: flex;
                justify-content: center;
                align-items: center;
                text-transform: uppercase;
                text-decoration: none;
            }
        "#)}>
            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            <Link<Route> to={Route::NotFound}>{ "404" }</Link<Route>>
        </div>
    }
}

