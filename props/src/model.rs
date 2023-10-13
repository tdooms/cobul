use std::ops::Deref;

use implicit_clone::unsync::IArray;
use yew::{Callback, hook, use_state, use_state_eq};
use yew::html::ImplicitClone;

#[derive(Clone, Debug, PartialEq)]
pub struct Model<T: Clone> {
    pub input: Callback<T>,
    pub value: T,
}

impl<T: Clone> ImplicitClone for Model<T> {}

impl<T: Clone> Model<T> {
    pub fn constant(value: T) -> Model<T> {
        Model { input: Callback::noop(), value }
    }

    pub fn emit(&self, value: T) {
        self.input.emit(value)
    }
}
impl<T: Clone + 'static> Model<T> {
    pub fn modify<I>(&self, function: impl Fn(T, I) -> T + 'static) -> Callback<I> {
        let Self { input, value } = self.clone();
        input.reform(move |inner| function(value.clone(), inner))
    }

    pub fn reform<I>(&self, function: impl Fn(I) -> T + 'static) -> Callback<I> {
        self.input.reform(function)
    }
}

impl<I: ImplicitClone> Model<IArray<I>> {
    fn mapper(self, index: usize) -> Model<I> {
        let array = self.value.clone();

        let reform = move |value| {
            let mut new = array.to_vec();
            new[index] = value;
            new.into()
        };

        Model { value: self.value[index].clone(), input: self.input.reform(reform) }
    }
    pub fn decompose(self) -> IArray<Model<I>> {
        self.value.iter().enumerate().map(|(idx, _)| self.clone().mapper(idx)).collect()
    }

    pub fn push(&self) -> Callback<I> {
        let Model { input, value: array } = self.clone();
        let reform = move |value| {
            let mut new = array.to_vec();
            new.push(value);
            new.into()
        };
        input.reform(reform)
    }
}

impl<T: Clone> Deref for Model<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[hook]
pub fn use_model<T: Clone + 'static, F: FnOnce() -> T>(f: F) -> Model<T> {
    let state = use_state(f);
    let cloned = state.clone();

    Model {
        input: Callback::from(move |new| cloned.set(new)),
        value: (*state).clone(),
    }
}

#[hook]
pub fn use_model_eq<T: Clone + PartialEq + 'static, F: FnOnce() -> T>(f: F) -> Model<T> {
    let state = use_state_eq(f);
    let cloned = state.clone();

    Model {
        input: Callback::from(move |new| cloned.set(new)),
        value: (*state).clone(),
    }
}

