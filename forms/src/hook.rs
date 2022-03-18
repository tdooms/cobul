use std::future::Future;

use validator::Validate;
use yew::virtual_dom::ListenerKind::onsubmit;
use yew::{hook, Callback, UseStateHandle};

use util::use_value_state;

use crate::errors::{use_errors, UseErrorHandle};

#[derive(PartialEq, Clone)]
pub struct FormActions<T> {
    onchange: Option<Callback<T>>,
    onsubmit: Option<Callback<T>>,
    oncancel: Option<Callback<T>>,
}

impl<T> Default for FormActions<T> {
    fn default() -> Self {
        Self {
            onchange: None,
            onsubmit: None,
            oncancel: None,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct UseFormHandle<T: Clone + Validate> {
    state: UseStateHandle<T>,
    actions: FormActions<T>,
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

    pub fn field<F>(&self, extract: impl Fn(&mut T) -> &mut F + 'static) -> Callback<F> {
        let Self {
            state,
            actions: FormActions { onchange, .. },
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
            actions: FormActions { onchange, .. },
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
            actions: FormActions { onchange, .. },
            errors,
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
pub fn use_form<T>(value: &T, actions: FormActions<T>) -> (UseFormHandle<T>, T)
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