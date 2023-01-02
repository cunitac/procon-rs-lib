use std::{
    cell::{Cell, UnsafeCell},
    ops::Deref,
};

pub struct LazyCell<T, F> {
    cell: OnceCell<T>,
    init: Cell<Option<F>>,
}

impl<T, F> LazyCell<T, F> {
    pub const fn new(init: F) -> Self {
        Self {
            cell: OnceCell::new(),
            init: Cell::new(Some(init)),
        }
    }
}

impl<T, F: FnOnce() -> T> Deref for LazyCell<T, F> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self.cell.get() {
            Some(val) => val,
            None => {
                self.cell.set(self.init.take().unwrap()()).ok().unwrap();
                self.cell.get().unwrap()
            }
        }
    }
}

pub struct OnceCell<T>(UnsafeCell<Option<T>>);

impl<T> OnceCell<T> {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }
    pub fn get(&self) -> Option<&T> {
        // SAFETY: None なら None だし、Some なら以後変更されないので OK
        unsafe { &*self.0.get() }.as_ref()
    }
    /// 2 回目以降の呼び出しでは `Err(val)` を返す
    pub fn set(&self, val: T) -> Result<(), T> {
        if self.get().is_some() {
            Err(val)
        } else {
            // SAFETY: ここでしか変更しないし、self.0.get() が None なのは確かめている
            *unsafe { &mut *self.0.get() } = Some(val);
            Ok(())
        }
    }
}
