#[derive(Clone)]
pub enum Msg {
    StateChange(StateChangeMessage),
    RouteChange(Page),
}

#[derive(Clone)]
pub enum StateChangeMessage {
    InputReceived,
    AddTodo,
    RemoveTodo(usize), //the contained usize is the index within the todos array!
    ShowAddTodo,
}

#[derive(Clone)]
pub enum Page {
    HomePage,
}
