use std::ops::Deref;

use implicit_clone::ImplicitClone;
use implicit_clone::sync::{IArray, IMap};
use yew::*;
use validator::Validate;

use cobul_props::{Model, use_model, use_model_eq};

fn validate(value: &impl Validate) -> IMap<&'static str, AttrValue> {
    value.validate()
        .err()
        .unwrap_or_default()
        .field_errors()
        .into_iter()
        .filter_map(|(k, v)| v.first().map(|v| v.message.clone()).flatten().map(|v| (k, v.into())))
        .collect()
}

pub trait Form: ImplicitClone {
    type Wrapper;
    fn from(state: State<Self>) -> Self::Wrapper;
}

#[derive(Clone, PartialEq)]
pub struct State<T: Form> {
    model: Model<T>,
    dirty: UseStateHandle<IArray<&'static str>>,
    errors: UseStateHandle<IMap<&'static str, AttrValue>>,
}

impl<T: Form> State<T> {
    pub fn error(&self, key: &'static str, filter_dirty: bool) -> Option<AttrValue> {
        self.errors.get(key).filter(|_| !filter_dirty || self.dirty.contains(&key))
    }
    pub fn errors(&self, filter_dirty: bool) -> IMap<&'static str, AttrValue> {
        let errors = (*self.errors).clone();
        errors.iter().filter(|(k, _)| !filter_dirty || self.dirty.contains(k)).collect()
    }
    pub fn valid(&self) -> bool {
        self.errors(false).is_empty()
    }
}

impl<T: Clone + Form> ImplicitClone for State<T> {}

impl<T: Form + Validate + 'static> State<T> {
    pub fn change<F>(&self, key: &'static str, map: impl Fn(&mut T) -> &mut F + 'static) -> Callback<F> {
        let Model { input, value: state } = self.model.clone();

        let errors = self.errors.clone();
        let dirty = self.dirty.clone();

        input.reform(move |value| {
            let mut new = (*dirty).to_vec();
            if !new.contains(&key) { new.push(key) }
            dirty.set(new.into());

            let mut new = state.clone();
            *map(&mut new) = value;
            errors.set(validate(&new));
            new
        })
    }
}

impl<T: Form> Deref for State<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.model.value }
}

#[hook]
pub fn use_form_with_model<T: Form + Default + Validate + 'static>(model: Model<T>) -> T::Wrapper {
    let dirty = use_state(IArray::default);
    let errors = use_state(|| validate(&model.value));

    Form::from(State { model, dirty, errors })
}

#[hook]
pub fn use_form<T: Form + Default + Validate + 'static>() -> T::Wrapper {
    let model = use_model(|| T::default());
    use_form_with_model(model)
}

#[hook]
pub fn use_form_eq<T: Form + Default + Validate + PartialEq + 'static>() -> T::Wrapper {
    let model = use_model_eq(|| T::default());
    use_form_with_model(model)
}