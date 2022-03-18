use std::collections::HashMap;
use validator::{Validate, ValidationError};
use yew::{hook, use_state, UseStateHandle};

#[derive(PartialEq, Clone)]
pub struct UseErrorHandle {
    map: HashMap<String, String>,
    state: UseStateHandle<HashMap<String, String>>,
}

impl UseErrorHandle {
    pub fn set(&self, key: &str, value: Option<String>) {
        let mut map = (*self.state).clone();
        match value {
            Some(value) => map.insert(key.to_owned(), value),
            None => map.remove(key),
        };
        self.state.set(map)
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let state = (*self.state).clone();
        state.get(key).or(self.map.get(key)).cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.state.is_empty() && self.map.is_empty()
    }

    pub fn all(&self) -> HashMap<String, String> {
        (*self.state)
            .clone()
            .into_iter()
            .chain(self.map.clone())
            .collect()
    }
}

// FIXME: this is far from efficient but I guess it works
// A few allocations every mouseclick is not terrible.
pub fn validate_fields(value: &impl Validate) -> HashMap<String, String> {
    let validated = match value.validate() {
        Ok(_) => Default::default(),
        Err(validated) => validated,
    };
    let errors = validated.field_errors();

    let map = |(field, errors): (&str, &Vec<ValidationError>)| match errors
        .first()
        .map(|err| err.message.clone().unwrap_or(err.code.clone()).into_owned())
    {
        Some(error) => Some((field.to_owned(), error)),
        None => None,
    };
    errors.into_iter().filter_map(map).collect()
}

#[hook]
pub fn use_errors<V>(value: &V) -> UseErrorHandle
where
    V: Validate,
{
    let state = use_state::<HashMap<String, String>, _>(|| Default::default());
    let map = validate_fields(value);
    UseErrorHandle { map, state }
}
