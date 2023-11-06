use std::ops::Deref;

use implicit_clone::unsync::IArray;
use yew::{AttrValue, Callback, hook, use_effect_with, use_memo, use_state, use_state_eq, UseStateHandle};
use yew::html::ImplicitClone;

#[derive(Clone, Debug, PartialEq)]
pub struct Model<T: Clone> {
    input: Option<Callback<T>>,
    value: T,
}

impl yew::html::IntoPropValue<Model<AttrValue>> for AttrValue {
    fn into_prop_value(self) -> Model<AttrValue> {
        Model::constant(self)
    }
}

impl yew::html::IntoPropValue<Model<AttrValue>> for &'static str {
    fn into_prop_value(self) -> Model<AttrValue> {
        Model::constant(self.into())
    }
}

impl yew::html::IntoPropValue<Model<u32>> for u32 {
    fn into_prop_value(self) -> Model<u32> {
        Model::constant(self)
    }
}

impl yew::html::IntoPropValue<Model<f32>> for f32 {
    fn into_prop_value(self) -> Model<f32> {
        Model::constant(self)
    }
}

impl yew::html::IntoPropValue<Model<bool>> for bool {
    fn into_prop_value(self) -> Model<bool> {
        Model::constant(self)
    }
}

impl<T: ImplicitClone> ImplicitClone for Model<T> {}

impl<T: Clone> Model<T> {
    pub fn new(value: T, input: Callback<T>) -> Model<T> {
        Model { input: Some(input), value }
    }
    pub fn constant(value: T) -> Model<T> {
        Model { input: None, value }
    }
    pub fn value(&self) -> T {
        self.value.clone()
    }
    pub fn input(&self) -> Option<Callback<T>> {
        self.input.clone()
    }
    pub fn emit(&self, value: T) {
        self.input.clone().unwrap_or_default().emit(value)
    }
    pub fn is_constant(&self) -> bool {
        self.input.is_none()
    }
}

impl<T: Clone + 'static> Model<Option<T>> {
    pub fn transpose(self) -> Option<Model<T>> {
        let cloned = self.clone();
        self.value.map(|value| Model { input: Some(cloned.reform(Some)), value })
    }
}

impl<T: Clone + 'static, E: Clone + 'static> Model<Result<T, E>> {
    pub fn transpose(self) -> Result<Model<T>, E> {
        let cloned = self.clone();
        self.value.map(|value| Model { input: Some(cloned.reform(Ok)), value })
    }
}

impl<T: Clone + 'static> Model<T> {
    pub fn modify<I>(&self, function: impl Fn(T, I) -> T + 'static) -> Callback<I> {
        let value = self.value.clone();
        self.reform(move |inner| function(value.clone(), inner))
    }

    pub fn reform<I>(&self, function: impl Fn(I) -> T + 'static) -> Callback<I> {
        match &self.input {
            Some(cb) => cb.reform(function),
            None => Callback::noop(),
        }
    }
}

impl<T: Clone + 'static> From<UseStateHandle<T>> for Model<T> {
    fn from(state: UseStateHandle<T>) -> Self {
        let value = (*state).clone();
        let input = Some(Callback::from(move |new| state.set(new)));

        Model { input, value }
    }
}

impl Model<bool> {
    pub fn toggle(&self) -> Callback<()> {
        let value = self.value.clone();
        self.reform(move |_| !value)
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

        Model { value: self.value[index].clone(), input: Some(self.reform(reform)) }
    }
    pub fn decompose(self) -> IArray<Model<I>> {
        self.value.iter().enumerate().map(|(idx, _)| self.clone().mapper(idx)).collect()
    }

    pub fn push(&self) -> Callback<I> {
        let array = self.value.clone();
        let reform = move |value| {
            let mut new = array.to_vec();
            new.push(value);
            new.into()
        };
        self.reform(reform)
    }
    pub fn remove(&self) -> Callback<usize> {
        let array = self.value.clone();
        let reform = move |value| {
            let mut new = array.to_vec();
            new.remove(value);
            new.into()
        };
        self.reform(reform)
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

