macro_rules! bool_prop {
    ($x:ident, $y:literal) => {
        #[derive(
            std::clone::Clone,
            std::fmt::Debug,
            std::cmp::PartialEq,
            std::default::Default,
            std::marker::Copy,
        )]
        #[doc(hidden)]
        pub struct $x(pub bool);

        impl $x {
            pub fn or(self, other: std::option::Option<Self>) -> Self {
                Self(self.0 || other.unwrap_or_default().0)
            }
        }

        impl std::convert::Into<yew::Classes> for $x {
            fn into(self) -> yew::Classes {
                match self.0 {
                    true => yew::Classes::from($y),
                    false => yew::Classes::new(),
                }
            }
        }

        impl yew::html::IntoPropValue<$x> for bool {
            fn into_prop_value(self) -> $x {
                $x(self)
            }
        }
    };
}

bool_prop!(Active, "is-active");
bool_prop!(Addons, "has-addons");
bool_prop!(Bordered, "is-bordered");
bool_prop!(Boxed, "is-boxed");
bool_prop!(Centered, "is-centered");
bool_prop!(Checked, "is-checked");
bool_prop!(Current, "is-current");
bool_prop!(Delete, "is-delete");
bool_prop!(Disabled, "is-disabled");
bool_prop!(HasDropdown, "has-dropdown");
bool_prop!(Expanded, "is-expanded");
bool_prop!(FixedSize, "has-fixed-size");
bool_prop!(Focused, "is-focused");
bool_prop!(Fullwidth, "is-fullwidth");
bool_prop!(Gapless, "is-gapless");
bool_prop!(Grouped, "is-grouped");
bool_prop!(GroupedMultiline, "is-grouped-multiline");
bool_prop!(Hidden, "is-hidden");
bool_prop!(Hoverable, "is-hoverable");
bool_prop!(Hovered, "is-hovered");
bool_prop!(Inverted, "is-inverted");
bool_prop!(Light, "is-light");
bool_prop!(Loading, "is-loading");
bool_prop!(Mobile, "is-mobile");
bool_prop!(Multiline, "is-multiline");
bool_prop!(Narrow, "is-narrow");
bool_prop!(Outlined, "is-outlined");
bool_prop!(Right, "is-right");
bool_prop!(Readonly, "is-readonly");
bool_prop!(Rounded, "is-rounded");
bool_prop!(Rtl, "is-rtl");
bool_prop!(Selected, "is-selected");
bool_prop!(Shadow, "has-shadow");
bool_prop!(Spaced, "is-spaced");
bool_prop!(Static, "is-static");
bool_prop!(Striped, "is-striped");
bool_prop!(TextCentered, "has-text-centered");
bool_prop!(Thin, "is-thin");
bool_prop!(Toggle, "is-toggle");
bool_prop!(ToggleRounded, "is-toggle-rounded");
bool_prop!(Transparent, "is-transparent");
bool_prop!(Up, "is-up");
bool_prop!(VCentered, "is-vcentered");
bool_prop!(Vertical, "is-vertical");
