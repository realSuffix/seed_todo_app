use crate::MyApp;
use seed::{*, prelude::*};
use crate::traits::component_trait::{ActionComponent, Component};
use crate::messages::{Msg, StateChangeMessage};
use super::todo_item_component::TodoItemComponent;
use super::add_item_component::AddItemComponent;

//The container for the todo items
pub struct ToDoContainerComponent;

impl Component<MyApp, Msg> for ToDoContainerComponent {
    fn view(model: &MyApp) -> Node<Msg> {
        if !model.show_add {
            div![
                id!("todo_wrapper"),
                div![
                    model.todos.iter().map(|todo| {
                        TodoItemComponent::view_with_props(&todo)
                    })
                ],
                AddItemComponent::view(model),
                div![
                    simple_ev(Ev::Click, Msg::StateChange(StateChangeMessage::ShowAddTodo)),
                    id!("btn_wrapper"),
                    i![
                        class!("fas fa-plus-circle")
                    ]
                ]
            ]
        } else {
            div![
                id!("todo_wrapper"),
                div![
                    model.todos.iter().map(|todo| {
                        TodoItemComponent::view_with_props(&todo)
                    })
                ],
                AddItemComponent::view(model),
            ]
        }
    }
}

