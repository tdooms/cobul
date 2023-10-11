use std::ops::Deref;

use cobul_props::{Model, use_model, use_model_eq};
use implicit_clone::ImplicitClone;
use yew::*;

pub trait Form: ImplicitClone {
    type Wrapper;
    fn from(state: State<Self>) -> Self::Wrapper;
}

pub struct State<T: Form> {
    model: Model<T>,
}

impl<T: Form + 'static> State<T> {
    pub fn change<F>(&self, _key: &'static str, map: impl Fn(&mut T) -> &mut F + 'static) -> Callback<F> {
        let Model { input, value: state } = self.model.clone();

        input.reform(move |value| {
            let mut new = state.clone();
            *map(&mut new) = value;
            new
        })
    }
}

impl<T: Form> Deref for State<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.model.value }
}

#[hook]
pub fn use_form_with_model<T: Form + Default + 'static>(model: Model<T>) -> T::Wrapper {
    Form::from(State { model })
}

#[hook]
pub fn use_form<T: Form + Default + 'static>() -> T::Wrapper {
    let model = use_model(|| T::default());
    Form::from(State { model })
}

#[hook]
pub fn use_form_eq<T: Form + Default + PartialEq + 'static>() -> T::Wrapper {
    let model = use_model_eq(|| T::default());
    Form::from(State { model })
}