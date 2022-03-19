mod components;
use components::{
    AppContainer::AppContainer, NavBar::NavBar, Hero::Hero, Skills::Skills, Footer::Footer
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
            <>
                <Hero />
                <Skills />
            </>
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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <NavBar />
                <AppContainer>
                    <Switch<Route> render={Switch::render(switch)} />
                </AppContainer>
                <Footer />
            </BrowserRouter>
        }
    }
}

fn main() {
   yew::start_app::<App>(); 
}
