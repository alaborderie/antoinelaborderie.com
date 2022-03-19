use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Hero)]
pub fn hero() -> Html {
    html! {
        <div class={css!(r#"
            display: flex;
            justify-content: space-around;
            align-items: center;
            height: calc(100vh - 75px);
            background-color: #303136;

            @media screen and (max-width: 600px) {
                flex-direction: column;
                justify-content: center;
                height: auto;
            }
        "#)}>
            <h1 class={css!(r#"
                font-size: 64px;
                width: 30vw;

                @media screen and (max-width: 600px) {
                    width: 80vw;
                    text-align: center;
                }
            "#)}>{ "Antoine Laborderie FullStack web developer" }</h1>
            <div class={css!(r#"
                width: 30vw;
                display: grid;
                gap: 50px;
                grid-template-columns: 1fr 1fr;
                padding: 32px;
                border-radius: 10px;
                background-color: rgb(210, 211, 215);

                img {
                    width: 150px;
                    height: 150px;
                    object-fit: contain;
                }

                @media screen and (max-width: 1200px) {
                    padding: 16px;
                    gap: 25px;

                    img {
                        width: 100px;
                        height: 100px;
                    }
                }

                @media screen and (max-width: 600px) {
                    padding: 32px;
                    width: auto;
                    margin-bottom: 64px;
                }
            "#)}>
            <img src="/public/rust-logo.svg" alt="Rust Logo" />
            <img src="/public/elixir-logo.png" alt="Elixir Logo" />
            <img src="/public/nodejs-logo.svg" alt="NodeJS Logo" />
            <img src="/public/react-logo.svg" alt="React Logo" />
            </div>
        </div>
    }
}

