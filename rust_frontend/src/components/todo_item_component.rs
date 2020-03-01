use crate::Todo;
use seed::{*, prelude::*};
use crate::messages::{Msg, StateChangeMessage};

//The single todo item
pub struct TodoItemComponent;

impl TodoItemComponent {
    pub fn view_with_props(todo: &Todo) -> Node<Msg> {
        div![
            class!("todo_item"),
            div![
                class!("todo_item_left"),
                format!("{}", todo.title)
            ],
            div![
                class!("todo_item_right"),
                i![
                    simple_ev(Ev::Click, Msg::StateChange(StateChangeMessage::RemoveTodo(todo.index))),
                    class!("fas fa-times")
                ],
            ]
        ]
    }
}
