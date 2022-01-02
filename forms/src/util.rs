use std::collections::HashMap;
use validator::{Validate, ValidationError};

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
