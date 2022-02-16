mod components;
use components::{
    AppContainer::AppContainer, NavBar::NavBar, Hero::Hero, Footer::Footer
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <AppContainer>
                <NavBar />
                <Hero />
                <Footer />
            </AppContainer>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App 
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}

fn main() {
   yew::start_app::<App>(); 
}
