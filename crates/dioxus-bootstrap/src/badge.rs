use dioxus::prelude::*;

use crate::types::{BadgeFill, Color};

/// Bootstrap Badge component.
///
/// # Bootstrap HTML → Dioxus
///
/// | HTML | Dioxus |
/// |---|---|
/// | `<span class="badge text-bg-primary">New</span>` | `Badge { color: Color::Primary, "New" }` |
/// | `<span class="badge rounded-pill text-bg-danger">99+</span>` | `Badge { color: Color::Danger, pill: true, "99+" }` |
/// | `<span class="badge bg-secondary">Alias</span>` | `Badge { color: Color::Secondary, fill: BadgeFill::Bg, "Alias" }` |
/// | `<span class="badge bg-info-subtle text-info-emphasis">2 fields</span>` | `Badge { color: Color::Info, fill: BadgeFill::Subtle, "2 fields" }` |
/// | `<span class="badge text-bg-secondary" role="button">Open</span>` | `Badge { color: Color::Secondary, onclick: move |_| {}, "Open" }` |
///
/// ```rust,no_run
/// # use dioxus::prelude::*;
/// # use dioxus_bootstrap_css::prelude::*;
/// # fn _doctest() -> Element {
/// rsx! {
///     Badge { color: Color::Primary, "New" }
///     Badge { color: Color::Danger, pill: true, "99+" }
///     Badge { color: Color::Info, fill: BadgeFill::Subtle, "2 fields" }
///     // Inside a heading
///     h1 { "Messages " Badge { color: Color::Info, "4" } }
/// }
/// # }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct BadgeProps {
    /// Badge color variant.
    #[props(default = Color::Primary)]
    pub color: Color,
    /// Which Bootstrap colour idiom to paint the badge with. Defaults to
    /// `text-bg-<color>`, so existing callers are unchanged.
    #[props(default)]
    pub fill: BadgeFill,
    /// Use pill (rounded) style.
    #[props(default)]
    pub pill: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Click event handler.
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Render as a real `<a class="badge" href=…>` instead of a `<span>`.
    ///
    /// Bootstrap documents badges used as links and buttons, and the anchor is
    /// not interchangeable with a span carrying a click handler: only the anchor
    /// is focusable, announced as a link, and openable in a new tab or copied
    /// from the context menu. A badge that navigates should be one.
    #[props(default)]
    pub href: Option<String>,
    /// Anchor `target` (e.g. `"_blank"`). Only applies when `href` is set.
    #[props(default)]
    pub target: Option<String>,
    /// Any additional HTML attributes.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    /// Child elements.
    pub children: Element,
}

/// Which element a badge renders as. Bootstrap documents badges used as links;
/// only a real anchor is focusable, announced as a link, and openable in a new
/// tab, so a span with a click handler is not a substitute for one.
fn badge_element(has_href: bool) -> &'static str {
    if has_href { "a" } else { "span" }
}

/// The badge's class string. Extracted so the colour idioms are assertable
/// without rendering — the whole point of the `fill` prop is which classes come
/// out, so that is what the tests pin.
fn badge_class(color: Color, fill: BadgeFill, pill: bool, class: &str) -> String {
    let color_classes = match fill {
        BadgeFill::TextBg => format!(" text-bg-{color}"),
        BadgeFill::Bg => format!(" bg-{color}"),
        BadgeFill::Subtle => format!(" bg-{color}-subtle text-{color}-emphasis"),
        BadgeFill::None => String::new(),
    };
    let pill = if pill { " rounded-pill" } else { "" };
    if class.is_empty() {
        format!("badge{color_classes}{pill}")
    } else {
        format!("badge{color_classes}{pill} {class}")
    }
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let full_class = badge_class(props.color, props.fill, props.pill, &props.class);

    if badge_element(props.href.is_some()) == "a" {
        let href = props.href.clone().expect("anchor form implies an href");
        let target = props.target.clone();
        return rsx! {
            a {
                class: "{full_class}",
                href: "{href}",
                target: target,
                onclick: move |evt| {
                    if let Some(handler) = &props.onclick {
                        handler.call(evt);
                    }
                },
                ..props.attributes,
                {props.children}
            }
        };
    }

    rsx! {
        span {
            class: "{full_class}",
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn badge_is_a_span_without_an_href() {
        assert_eq!(badge_element(false), "span");
    }

    #[test]
    fn badge_with_an_href_is_an_anchor() {
        assert_eq!(badge_element(true), "a");
    }

    #[test]
    fn badge_default_fill_is_text_bg() {
        // The default must stay `text-bg-*`: it is what every existing caller
        // renders today, so the new prop has to be invisible when unset.
        assert_eq!(
            badge_class(Color::Primary, BadgeFill::default(), false, ""),
            "badge text-bg-primary"
        );
    }

    #[test]
    fn badge_bg_fill_omits_the_foreground() {
        assert_eq!(
            badge_class(Color::Secondary, BadgeFill::Bg, false, ""),
            "badge bg-secondary"
        );
    }

    #[test]
    fn badge_subtle_fill_emits_both_halves_of_the_pair() {
        // Subtle is one idiom, not two utilities: the background is unreadable
        // without the emphasis foreground, so both must always appear together.
        assert_eq!(
            badge_class(Color::Info, BadgeFill::Subtle, false, ""),
            "badge bg-info-subtle text-info-emphasis"
        );
    }

    #[test]
    fn badge_none_fill_emits_geometry_only() {
        assert_eq!(
            badge_class(Color::Primary, BadgeFill::None, false, ""),
            "badge"
        );
    }

    #[test]
    fn badge_pill_and_extra_classes_survive_every_fill() {
        assert_eq!(
            badge_class(Color::Danger, BadgeFill::TextBg, true, "ms-2"),
            "badge text-bg-danger rounded-pill ms-2"
        );
        assert_eq!(
            badge_class(Color::Danger, BadgeFill::None, true, "ms-2"),
            "badge rounded-pill ms-2"
        );
    }
}
