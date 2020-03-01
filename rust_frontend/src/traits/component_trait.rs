use seed::prelude::{Node, Orders};

pub trait ActionComponent<'a, A, M: 'static>
{
    fn view(model: &A) -> Node<M>;
    fn handle(msg: M, model: &'a mut A, _: &'a mut impl Orders<M>);
}

pub trait Component<A, M>
{
    fn view(model: &A) -> Node<M>;
}
