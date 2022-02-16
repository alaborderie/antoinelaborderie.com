use yew::{Component, Children, Context, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct AppContainerProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct AppContainer;

impl Component for AppContainer {
    type Message = ();
    type Properties = AppContainerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="app-container">
               { for ctx.props().children.iter() }
            </div>
        }
    }
}
