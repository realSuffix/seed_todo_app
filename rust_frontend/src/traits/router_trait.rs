use seed::prelude::{Node, Orders};
use seed::Url;

pub trait Router<'a, A, M: 'static>
{
    fn view(model: &A) -> Node<M>;
    fn handle(msg: M, model: &'a mut A, _: &'a mut impl Orders<M>);
    fn routes(url: Url) -> Option<M>;
}

