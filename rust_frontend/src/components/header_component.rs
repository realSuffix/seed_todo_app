use crate::MyApp;
use seed::{*, prelude::*};
use crate::traits::component_trait::Component;
use crate::messages::Msg;

//Header
pub struct HeaderComponent;

impl Component<MyApp, Msg> for HeaderComponent {
    fn view(_model: &MyApp) -> Node<Msg> {
        div![
            id!("header"),
            "Todo App"
        ]
    }
}
