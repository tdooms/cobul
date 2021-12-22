use yew::{Callback, Properties};
use std::collections::HashMap;
use validator::{Validate, ValidationError};

#[derive(Properties, PartialEq, Clone)]
pub struct Form<T: 'static + Validate + PartialEq + Clone, C: 'static + PartialEq> {
    pub inner: T,
    pub callback: Callback<C>
}

impl<T: 'static + Validate + PartialEq + Clone, C: 'static + PartialEq> Form<T, C> {
    // FIXME: this is far from efficient but I guess it works
    // A few allocations every mouseclick is not terrible.
    pub fn errors(&self) -> HashMap<String, String> {
        let validated = match self.inner.validate() {
            Ok(_) => Default::default(),
            Err(validated) => validated
        };
        let errors = validated.field_errors();

        let map = |(field, errors): (&str, &Vec<ValidationError>)| {
            match errors.first().map(|err| err.message.clone().unwrap_or(err.code.clone()).into_owned()) {
                Some(error) => Some((field.to_owned(), error)),
                None => None
            }
        };

        errors.into_iter().filter_map(map).collect()
    }

    pub fn values(&self) -> T {
        self.inner.clone()
    }

    pub fn field<P>(&self, func: impl Fn(P, &mut T) + 'static, variant: impl Fn(T) -> C + 'static) -> Callback<P> {
        let cloned = self.inner.clone();
        self.callback.reform(move |p| {
            let mut inner = cloned.clone();
            func(p, &mut inner);
            variant(inner)
        })
    }

    pub fn callback(&self, variant: impl Fn(T) -> C + 'static) -> Callback<()> {
        let cloned = self.inner.clone();
        self.callback.reform(move |_| variant(cloned.clone()))
    }
}