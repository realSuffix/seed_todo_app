use crate::MyApp;
use seed::{*, prelude::*};
use crate::traits::component_trait::Component;
use crate::components::homepage_component::HomePageComponent;
use crate::messages::{Page, Msg};
use crate::traits::router_trait::Router;

//Router Component
pub struct RouterComponent;

impl<'a> Router<'a, MyApp, Msg> for RouterComponent {
    fn view(model: &MyApp) -> Node<Msg> {
        match model.current_page {
            Page::HomePage => HomePageComponent::view(model),
        }
    }

    fn handle(msg: Msg, model: &'a mut MyApp, _: &'a mut impl Orders<Msg>) {
        if let Msg::RouteChange(page) = msg {
            match page {
                Page::HomePage => {
                    seed::push_route(vec![""]);
                }
            };
            model.current_page = page;
        }
    }

    fn routes(_url: Url) -> Option<Msg> {
        return Some(Msg::RouteChange(Page::HomePage));
    }
}
