use std::collections::HashMap;
use std::future::Future;

use validator::Validate;
use yew::{hook, Callback, UseStateHandle};

use hooks::use_value_state;

use crate::actions::Actions;
use crate::errors::{use_errors, UseErrorHandle};

#[derive(Clone, PartialEq)]
pub struct UseFormHandle<T: Clone + Validate> {
    state: UseStateHandle<T>,
    actions: Actions<T>,
    errors: UseErrorHandle,
}

impl<T: Clone + Validate + 'static> UseFormHandle<T> {
    pub fn submit(&self) -> Callback<()> {
        let state = self.state.clone();
        self.actions
            .onsubmit
            .clone()
            .unwrap_or_default()
            .reform(move |_| (*state).clone())
    }

    pub fn cancel(&self) -> Callback<()> {
        let state = self.state.clone();
        self.actions
            .oncancel
            .clone()
            .unwrap_or_default()
            .reform(move |_| (*state).clone())
    }

    pub fn error(&self, key: &str) -> Option<String> {
        self.errors.get(key)
    }

    pub fn errors(&self) -> HashMap<String, String> {
        self.errors.all()
    }

    pub fn can_submit(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn field<F>(&self, extract: impl Fn(&mut T) -> &mut F + 'static) -> Callback<F> {
        let Self {
            state,
            actions: Actions { onchange, .. },
            ..
        } = self.clone();

        Callback::from(move |value| {
            let mut cloned = (*state).clone();
            *extract(&mut cloned) = value;
            if let Some(cb) = onchange.clone() {
                cb.emit(cloned.clone())
            }
            state.set(cloned);
        })
    }

    pub fn maybe_field<F>(
        &self,
        extract: impl Fn(&mut T) -> &mut F + 'static,
        field: &'static str,
        explain: impl Fn(String) -> String + 'static,
    ) -> Callback<Result<F, String>> {
        let Self {
            state,
            actions: Actions { onchange, .. },
            errors,
        } = self.clone();

        Callback::from(move |result| match result {
            Ok(value) => {
                let mut cloned = (*state).clone();
                *extract(&mut cloned) = value;
                if let Some(cb) = onchange.clone() {
                    cb.emit(cloned.clone())
                }
                state.set(cloned);
                errors.set(field.clone(), None);
            }
            Err(err) => errors.set(field.clone(), Some(explain(err))),
        })
    }

    pub fn async_field<F: 'static, Fut>(
        &self,
        func: impl Fn(T, F) -> Fut + Clone + 'static,
    ) -> Callback<F>
    where
        Fut: Future<Output = T>,
    {
        let Self {
            state,
            actions: Actions { onchange, .. },
            ..
        } = self.clone();

        Callback::from(move |f| {
            let state = state.clone();
            let func = func.clone();
            let onchange = onchange.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let cloned = func((*state).clone(), f).await;
                if let Some(cb) = onchange.clone() {
                    cb.emit(cloned.clone())
                }
                state.set(cloned);
            });
        })
    }
}

#[hook]
pub fn use_form<T>(value: &T, actions: Actions<T>) -> (UseFormHandle<T>, T)
where
    T: Clone + Validate + PartialEq + 'static,
{
    let state = use_value_state(value);

    let cloned = (*state).clone();
    let errors = use_errors(&cloned);

    let handle = UseFormHandle {
        state,
        errors,
        actions,
    };

    (handle, cloned)
}
