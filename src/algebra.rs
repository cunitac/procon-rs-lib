use std::fmt::Debug;

pub trait Element: Sized + Clone + Debug {}
impl<T: Sized + Clone + Debug> Element for T {}

pub trait Monoid {
    type Item: Element;
    fn id() -> Self::Item;
    fn prod(a: &Self::Item, b: &Self::Item) -> Self::Item;
    fn op_from_left(a: &Self::Item, b: &mut Self::Item) {
        *b = Self::prod(a, b);
    }
    fn op_from_right(a: &mut Self::Item, b: &Self::Item) {
        *a = Self::prod(a, b);
    }
    fn fold<'a, I>(iterable: I) -> Self::Item
    where
        I: IntoIterator<Item = &'a Self::Item>,
        Self::Item: 'a,
    {
        iterable
            .into_iter()
            .fold(Self::id(), |a, b| Self::prod(&a, b))
    }
}

pub trait Action {
    type Item: Element;
    type Operator: Element;
    fn image(item: &Self::Item, op: &Self::Operator) -> Self::Item;
    fn act(item: &mut Self::Item, op: &Self::Operator) {
        *item = Self::image(item, op);
    }
}

#[macro_export]
macro_rules! define_monoid {
    (type $t:ident = ($item:ty, $op:expr, $id:expr)) => {
        enum $t {}
        impl Monoid for $t {
            type Item = $item;
            fn prod(a: &$item, b: &$item) -> $item {
                $op(a, b)
            }
            fn id() -> $item {
                $id
            }
        }
    };
}
