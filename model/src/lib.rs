use std::ops::Deref;

use implicit_clone::unsync::IArray;
use yew::{Callback, hook, use_effect_with, use_memo, use_state, use_state_eq, UseStateHandle};
use yew::html::ImplicitClone;

#[derive(Clone, Debug, PartialEq)]
pub struct Model<T: Clone> {
    pub input: Callback<T>,
    pub value: T,
}

impl<T: ImplicitClone> ImplicitClone for Model<T> {}

impl<T: Clone> Model<T> {
    pub fn constant(value: T) -> Model<T> {
        Model { input: Callback::noop(), value }
    }

    pub fn emit(&self, value: T) {
        self.input.emit(value)
    }
}

impl<T: Clone + 'static> Model<Option<T>> {
    pub fn transpose(self) -> Option<Model<T>> {
        self.value.map(|value| Model { input: self.input.reform(Some), value })
    }
}

impl<T: Clone + 'static, E: Clone + 'static> Model<Result<T, E>> {
    pub fn transpose(self) -> Result<Model<T>, E> {
        self.value.map(|value| Model { input: self.input.reform(Ok), value })
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

impl<T: Clone + 'static> From<UseStateHandle<T>> for Model<T> {
    fn from(state: UseStateHandle<T>) -> Self {
        let value = (*state).clone();
        let input = Callback::from(move |new| state.set(new));

        Model { input, value }
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
    pub fn remove(&self) -> Callback<usize> {
        let Model { input, value: array } = self.clone();
        let reform = move |value| {
            let mut new = array.to_vec();
            new.remove(value);
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
    use_state(f).into()
}

#[hook]
pub fn use_model_eq<T: Clone + PartialEq + 'static, F: FnOnce() -> T>(f: F) -> Model<T> {
    use_state_eq(f).into()
}

#[hook]
pub fn use_model_with<D: PartialEq + Clone + 'static, T: Clone + 'static, F: Fn(&D) -> T + 'static>(deps: D, f: F) -> Model<T> {
    let output = use_memo(deps.clone(), |deps| f(deps));
    let state = use_state(|| (*output).clone());

    let cloned = state.clone();
    use_effect_with(deps, move |_| {
        cloned.set((*output).clone());
    });

    state.into()
}

