use std::rc::Rc;
use yew::Callback;

#[derive(PartialEq, Clone)]
pub struct Actions<T> {
    pub onchange: Option<Callback<Rc<T>>>,
    pub onsubmit: Option<Callback<Rc<T>>>,
    pub oncancel: Option<Callback<Rc<T>>>,
}

impl<T> Actions<T> {
    pub fn new() -> Self {
        Self {
            onchange: None,
            onsubmit: None,
            oncancel: None,
        }
    }
    pub fn submit(mut self, cb: Callback<Rc<T>>) -> Self {
        self.onsubmit = Some(cb);
        self
    }
    pub fn change(mut self, cb: Callback<Rc<T>>) -> Self {
        self.onchange = Some(cb);
        self
    }
    pub fn cancel(mut self, cb: Callback<Rc<T>>) -> Self {
        self.oncancel = Some(cb);
        self
    }
}
