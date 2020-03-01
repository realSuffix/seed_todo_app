use crate::MyApp;
use seed::{*, prelude::*};
use crate::traits::component_trait::Component;
use crate::messages::Msg;
use super::header_component::HeaderComponent;
use super::item_wrapper_component::ToDoContainerComponent;

//The entire page
pub struct HomePageComponent;

impl Component<MyApp, Msg> for HomePageComponent {
    fn view(model: &MyApp) -> Node<Msg> {
        div![
            class!{"page_root"},
            HeaderComponent::view(model), //show header
            ToDoContainerComponent::view(model), //show items
        ]
    }
}
