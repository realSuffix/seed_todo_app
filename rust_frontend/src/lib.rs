use seed::{*, prelude::*};

mod router;
mod messages;
mod traits;
mod components;

use router::RouterComponent;
use crate::traits::component_trait::{Component, ActionComponent};
use crate::messages::{Page, Msg, StateChangeMessage};
use crate::traits::router_trait::Router;
use web_sys::HtmlInputElement;
use crate::components::add_item_component::AddItemComponent;

pub struct MyApp {
    pub val: i32,
    pub current_page: Page,
    pub todos: Vec<Todo>,
    pub refs: Refs,
    pub new_todo_content: String,
    pub show_add: bool,
}

#[derive(Default)]
pub struct Refs {
    pub editing_todo_input: ElRef<HtmlInputElement>,
}

#[allow(dead_code)]
pub struct Todo {
    title: String,
    completed: bool,
    index: usize,
}

impl Component<MyApp, Msg> for MyApp {
    fn view(model: &MyApp) -> Node<Msg> {
        RouterComponent::view(model)
    }
}

impl MyApp {
    fn update(msg: Msg, model: &mut MyApp, orders: &mut impl Orders<Msg>) {
        match msg.clone() {
            Msg::RouteChange(_) => {
                RouterComponent::handle(msg, model, orders);
            }
            Msg::StateChange(state) => {
                match state {
                    StateChangeMessage::InputReceived => {
                        let input_ref = model.refs.editing_todo_input.get().unwrap();
                        model.new_todo_content = input_ref.value();
                    }
                    StateChangeMessage::AddTodo => {
                        AddItemComponent::handle(msg, model, orders);
                    }
                    StateChangeMessage::ShowAddTodo => {
                        model.show_add = true;
                    }
                    StateChangeMessage::RemoveTodo(index) => {
                        model.todos.remove(index);
                        //update indexes
                        for i in 0..model.todos.len() {
                            model.todos[i].index = i;
                        }
                    }
                }
            }
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            val: 0,
            current_page: Page::HomePage,
            todos: vec![
                Todo {
                    title: "First Todo".to_owned(),
                    completed: false,
                    index: 0,
                },
                Todo {
                    title: "Second Todo".to_owned(),
                    completed: true,
                    index: 1,
                },
                Todo {
                    title: "Third Todo".to_owned(),
                    completed: false,
                    index: 2,
                },
            ],
            refs: Refs::default(),
            new_todo_content: String::new(),
            show_add: false,
        }
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(MyApp::update, MyApp::view)
        .routes(RouterComponent::routes)
        .build_and_start();
}