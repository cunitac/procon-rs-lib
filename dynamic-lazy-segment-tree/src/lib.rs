pub trait Type {
    type Item: Clone;
    type Operator: Clone;
    fn id() -> Self::Item;
    fn prod(a: &Self::Item, b: &Self::Item) -> Self::Item;
    fn composition(a: &Self::Operator, b: &Self::Operator) -> Self::Operator;
    fn operate_with_len(val: &mut Self::Item, op: &Self::Operator, len: usize);
    /// 長さ `1` と見なす．
    fn operate(val: &mut Self::Item, op: &Self::Operator) {
        Self::operate_with_len(val, op, 1)
    }
}

enum Node<T: Type> {
    None,
    Leaf {
        index: usize,
        value: T::Item,
    },
    Span {
        prod: T::Item,
        lazy: Option<T::Operator>,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl<T: Type> Node<T> {
    fn propagate(&mut self, l: usize, r: usize) {
        if let Self::Span { prod, lazy, left, right } = self {
            if let Some(lazy) = lazy.take() {
                T::operate_with_len(prod, &lazy, r - l);
                left.compose_lazy(&lazy);
                right.compose_lazy(&lazy);
            }
        }
    }
    fn compose_lazy(&mut self, op: &T::Operator) {
        match self {
            Self::None { .. } => {}
            Self::Leaf { value, .. } => T::operate(value, op),
            Self::Span { lazy: Some(lazy), .. } => *lazy = T::composition(lazy, op),
            Self::Span { lazy, .. } => *lazy = Some(op.clone()),
        }
    }
    fn prod(&mut self, l: usize, r: usize) -> Option<&T::Item> {
        self.propagate(l, r);
        match self {
            Self::None { .. } => None,
            Self::Leaf { value, .. } => Some(value),
            Self::Span { prod, .. } => Some(prod),
        }
    }
    fn get(&mut self, i: usize, l: usize, r: usize) -> Option<&T::Item> {
        self.propagate(l, r);
        match self {
            Self::Leaf { index, value, .. } if i == *index => Some(value),
            Self::Span { left, right, .. } => {
                let mid = (l + r) / 2;
                if i < mid {
                    left.get(i, l, mid)
                } else {
                    right.get(i, mid, r)
                }
            }
            _ => None,
        }
    }
    fn modify(&mut self, i: usize, f: impl FnOnce(&mut T::Item), l: usize, r: usize) {
        self.propagate(l, r);
        match self {
            Self::None { .. } => {
                let mut value = T::id();
                f(&mut value);
                *self = Self::Leaf { index: i, value }
            }
            Self::Leaf { index, value } => {
                if i == *index {
                    return f(value);
                }
                let is_left = *index < (l + r) / 2;
                let leaf = std::mem::replace(self, Self::None); // None is temporary
                let (left, right) = if is_left { (leaf, Self::None) } else { (Self::None, leaf) };
                *self = Self::Span {
                    prod: T::id(), // temporary
                    lazy: None,
                    left: Box::new(left),
                    right: Box::new(right),
                };
                self.modify(i, f, l, r);
                self.update();
            }
        }
    }
}
