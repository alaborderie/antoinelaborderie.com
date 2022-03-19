
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Skills)]
pub fn skills() -> Html {
    html! {
        <div class={css!(r#"
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            background-color: #202124;
            padding: 64px;
            p {
                margin: 0;
                @media screen and (max-width: 600px) {
                    text-align: center;
                }
            }
        "#)}>
            <h2 class={css!(r#"
                font-size: 32px;
                font-weight: bold;
            "#)}>{ "Un projet web ?" }</h2>
            <p>{ "Contactez-moi sur " }<a class={css!(r#"
            text-decoration: none;
            color: lightblue;
            "#)} href="https://www.linkedin.com/in/antoine-laborderie-866090130">{ "Linkedin" }</a>{ " pour m'en parler." }</p>
            <p>{ "Toujours à jour sur les nouveaux frameworks, je saurai m'adapter à votre stack ou vous proposer celle qui sera adaptée à votre besoin. "}</p>
            <p>{ "Après plus de 5 ans d'expérience avec Javascript, que ce soit avec Angular, React ou NodeJS, il y a de grandes chances que mes compétences puissent vous aider dans votre projet. "}</p>
            <p>{ "Un autre langage ? Rust ? Elixir ? Mes projets perso me permettent de me former sur ces technos." }</p>
            <p class={css!(r#"
                font-size: 8px;
                margin-top: 8px !important;
                font-style: italic;
                "#)}>{ "Psssst... ce site web est fait avec Rust / Yew !" }</p>
        </div>
    }
}

