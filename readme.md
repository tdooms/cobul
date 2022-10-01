# Cobul
Cobul is a library of components for the [Bulma](https://bulma.io/) CSS framework, written in [Yew](https://yew.rs/).

## Overview
The library consists of different parts:
- `base`: The core library, containing all standard components described on the website.
- `extensions`: A collection of components from some well-known bulma extensions.
- `fa`: Enums containing all free icons from [Font Awesome](https://fontawesome.com/).
- `simple`: A collection of 'higher-level' components to simplify the creation of common structures.

## General use
To improve the user experience of this library. Everything is kept as close as possible to the original Bulma components. 

```html
<button class="button is-primary is-large is-light " disabled> White </button
```
Is written like this in Cobul:

```rust
<Button color={Color::Primary} size={Size::Large} light=true disabled=true> {"White"} </Button>
```

As you can see, there are only a few changes to make the library feel 'rusty' and easy to use within yew:
- Attributes which are enum-like, like colors and sizes are defined as rust enums.
- Attributes which are boolean, are defined as bools without the **is-** or **are-** prefixes.
- Certain constructions are slightly simplified, like the Icon component, shown in the example below.
- Form components always have a callback called *input* and value attribute called *value* for two-way coupling.
- Additionally, every form component has a *model* attribute to set both the *input* and *value*, like in Vue (see use_model for why this is handy).
- Some components have non-standard behaviour to make them more usable, like the *Dropdown* component, which has a *focus* callback know when it is focussed or blurred.

Every component has a *class* and *style* attribute for easier styling.
Sadly, they don't have every standard event handler (like onkeypress, [reason](https://github.com/yewstack/yew/issues/1533)).

```html
<button class="button is-primary">
    <span class="icon"> <i class="fab fa-twitter"></i> </span>
    <span>@jgthms</span>
</button>
```

Is written as follows in Cobul:

```rust
<Button color={Color::Primary}>
    <Icon icon={fa::Brands::Twitter} /> // using the fa feature, notice the missing <i> tag
    <span> {"@jgthms"} </span>
</Button>
```

or simplified using the simple components feature:

```rust
<simple::Button color={Color::Primary} icon={fa::Brands::Twitter} text={"@jgthms"} />
```

Callbacks are defined as follows:

```rust
<Textarea input={modify_stuff} />
<Button click={submit_stuff} />
```

## Base
