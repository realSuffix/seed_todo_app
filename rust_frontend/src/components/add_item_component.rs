use crate::{MyApp, Todo};
use seed::{*, prelude::*};
use crate::traits::component_trait::ActionComponent;
use crate::messages::{Msg, StateChangeMessage};

//Component for adding items
pub struct AddItemComponent;

impl<'a> ActionComponent<'a, MyApp, Msg> for AddItemComponent {
    fn view(model: &MyApp) -> Node<Msg> {
        if model.show_add {
            div![
                class!("add_item"),
                div![
                    class!("add_item_left"),
                    input![
                        el_ref(&model.refs.editing_todo_input),
                        simple_ev(Ev::Input, Msg::StateChange(StateChangeMessage::InputReceived)),
                    ],
                ],
                div![
                    class!("add_item_right"),
                    button![
                        class!("add_todo_button"),
                        "Add todo",
                        simple_ev(Ev::Click, Msg::StateChange(StateChangeMessage::AddTodo))
                    ]
                ]
            ]
        } else {
            empty![]
        }
    }

    fn handle(_: Msg, model: &'a mut MyApp, _: &'a mut impl Orders<Msg>) {
        if !model.new_todo_content.is_empty() {
            model.todos.push(
                Todo {
                    title: model.new_todo_content.clone(),
                    completed: false,
                    index: model.todos.len(),
                }
            );
            model.new_todo_content.clear();
            model.show_add = false;
        }
    }
}

