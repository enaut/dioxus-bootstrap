use dioxus::prelude::*;

/// A single slide in the Carousel.
#[derive(Clone, PartialEq)]
pub struct CarouselSlide {
    /// Image source URL.
    pub src: String,
    /// Alt text for the image.
    pub alt: String,
    /// Optional caption title.
    pub caption_title: Option<String>,
    /// Optional caption text.
    pub caption_text: Option<String>,
}

/// Bootstrap Carousel component — signal-driven, no JavaScript.
///
/// ```rust
/// let active = use_signal(|| 0usize);
/// rsx! {
///     Carousel {
///         active: active,
///         slides: vec![
///             CarouselSlide { src: "/img/1.jpg".into(), alt: "First".into(),
///                 caption_title: Some("First slide".into()), caption_text: None },
///             CarouselSlide { src: "/img/2.jpg".into(), alt: "Second".into(),
///                 caption_title: None, caption_text: None },
///         ],
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct CarouselProps {
    /// Signal controlling the active slide index.
    pub active: Signal<usize>,
    /// Slide definitions.
    pub slides: Vec<CarouselSlide>,
    /// Show indicator dots.
    #[props(default = true)]
    pub indicators: bool,
    /// Show prev/next controls.
    #[props(default = true)]
    pub controls: bool,
    /// Crossfade transition instead of slide.
    #[props(default)]
    pub fade: bool,
    /// Dark variant for lighter background images.
    #[props(default)]
    pub dark: bool,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let current = *props.active.read();
    let mut active_signal = props.active;
    let total = props.slides.len();

    if total == 0 {
        return rsx! {};
    }

    let mut classes = vec!["carousel".to_string(), "slide".to_string()];
    if props.fade {
        classes.push("carousel-fade".to_string());
    }
    if props.dark {
        classes.push("carousel-dark".to_string());
    }
    if !props.class.is_empty() {
        classes.push(props.class.clone());
    }
    let full_class = classes.join(" ");

    rsx! {
        div { class: "{full_class}",
            // Indicators
            if props.indicators {
                div { class: "carousel-indicators",
                    for i in 0..total {
                        button {
                            class: if current == i { "active" } else { "" },
                            r#type: "button",
                            "aria-current": if current == i { "true" } else { "false" },
                            "aria-label": "Slide {i}",
                            onclick: move |_| active_signal.set(i),
                        }
                    }
                }
            }

            // Slides
            div { class: "carousel-inner",
                for (i, slide) in props.slides.iter().enumerate() {
                    div {
                        class: if current == i { "carousel-item active" } else { "carousel-item" },
                        img {
                            class: "d-block w-100",
                            src: "{slide.src}",
                            alt: "{slide.alt}",
                        }
                        if slide.caption_title.is_some() || slide.caption_text.is_some() {
                            div { class: "carousel-caption d-none d-md-block",
                                if let Some(ref title) = slide.caption_title {
                                    h5 { "{title}" }
                                }
                                if let Some(ref text) = slide.caption_text {
                                    p { "{text}" }
                                }
                            }
                        }
                    }
                }
            }

            // Controls
            if props.controls && total > 1 {
                button {
                    class: "carousel-control-prev",
                    r#type: "button",
                    onclick: move |_| {
                        let prev = if current == 0 { total - 1 } else { current - 1 };
                        active_signal.set(prev);
                    },
                    span { class: "carousel-control-prev-icon", "aria-hidden": "true" }
                    span { class: "visually-hidden", "Previous" }
                }
                button {
                    class: "carousel-control-next",
                    r#type: "button",
                    onclick: move |_| {
                        let next = if current + 1 >= total { 0 } else { current + 1 };
                        active_signal.set(next);
                    },
                    span { class: "carousel-control-next-icon", "aria-hidden": "true" }
                    span { class: "visually-hidden", "Next" }
                }
            }
        }
    }
}
