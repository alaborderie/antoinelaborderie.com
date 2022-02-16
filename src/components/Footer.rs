use stylist::yew::{styled_component};
use yew::prelude::*;

#[styled_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class={css!(r#"
            width: 100%;
            height: 200px;
            background-color: #2d2d2d;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;

            .nav-footer {
                display: flex;
                justify-content: center;
                align-items: center;
                
                
            }
            .nav-footer a {
                color: white;
                margin: 10px;
                padding: 15px;
                display: flex;
                justify-content: center;
                align-items: center;
            }

            p {
                color: white;
            }

            p a {
                color: white;
            }
        "#)}>
            <div class="nav-footer">
                <a class="nav-footer-text" href="https://github.com/alaborderie">{ "Github" }</a>
                <a class="nav-footer-text" href="https://twitter.com/Antoine64480">{ "Twitter" }</a>
                <a class="nav-footer-text" href="https://www.linkedin.com/in/antoine-laborderie-866090130">{ "Linkedin" }</a>
            </div>
            <p>{ "Written in Rust using " }<a href="https://github.com/yewstack/yew">{ "Yew" }</a></p>
        </div>
    }
}
