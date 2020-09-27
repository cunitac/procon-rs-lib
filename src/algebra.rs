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

/// 右作用
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

#[macro_export]
macro_rules! define_action {
    (type $t:ident = |$a:ident: $item:ty, $b:ident, $op:ty| $image:expr) => {
        enum $t {}
        impl Action for $t {
            type Item = $item;
            type Operator = $op;
            fn image($a: &$item, $b: &$op) -> $item {
                $op
            }
        }
    };
}

/// # Example
/// ```
/// define_for_lazy_seg_tree! {
///     impl (Monoid, Monoid, Action) for (M, O, A) {
///         type Item = u64;
///         type Operator = (u64, u64);
///         fn id_it() = 0;
///         fn prod_it(a, b) = a + b;
///         fn id_op() = (1, 0);
///         fn prod_op(a, b) = (a.0 * b.0, a.1 * b.0 + b.1);
///         fn image(a, b) = a * b.0 + b.1;
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_monoids_action {
    (
        impl (Monoid, Monoid, Action) for ($m:ident, $o:ident, $a:ident) {
            type Item = $it:ty;
            type Operator = $op:ty;
            fn id_it() = $ii_res:expr;
            fn prod_it($pi_a:ident, $pi_b:ident) = $pi_res:expr;
            fn id_op() = $io_res:expr;
            fn prod_op($po_a:ident, $po_b:ident) = $po_res:expr;
            fn image($im_a:ident, $im_b:ident) = $im_res:expr;
        }
    ) => {
        enum $m {}
        enum $o {}
        enum $a {}
        impl crate::algebra::Monoid for $m {
            type Item = $it;
            fn id() -> $it {
                $ii_res
            }
            fn prod($pi_a: &$it, $pi_b: &$it) -> $it {
                $pi_res
            }
        }
        impl crate::algebra::Monoid for $o {
            type Item = $op;
            fn id() -> $op {
                $io_res
            }
            fn prod($po_a: &$op, $po_b: &$op) -> $op {
                $po_res
            }
        }
        impl crate::algebra::Action for $a {
            type Item = $it;
            type Operator = $op;
            fn image($im_a: &$it, $im_b: &$op) -> $it {
                $im_res
            }
        }
    };
}
