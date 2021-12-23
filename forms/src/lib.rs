use yew::{Callback, Properties};
use std::collections::HashMap;
use validator::{Validate, ValidationError};

#[derive(Properties, PartialEq, Clone)]
pub struct Form<Ty: 'static + Validate + PartialEq + Clone, Call: 'static + PartialEq> {
    pub inner: Ty,
    pub callback: Callback<Call>
}

impl<Ty: 'static + Validate + PartialEq + Clone, Call: 'static + PartialEq> Form<Ty, Call> {
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

    pub fn values(&self) -> Ty {
        self.inner.clone()
    }

    pub fn field<Field>(&self, func: impl Fn(Field, &mut Ty) + 'static, variant: impl Fn(Ty) -> Call + 'static) -> Callback<Field> {
        let cloned = self.inner.clone();
        self.callback.reform(move |p| {
            let mut inner = cloned.clone();
            func(p, &mut inner);
            variant(inner)
        })
    }

    pub fn callback(&self, variant: impl Fn(Ty) -> Call + 'static) -> Callback<()> {
        let cloned = self.inner.clone();
        self.callback.reform(move |_| variant(cloned.clone()))
    }
}