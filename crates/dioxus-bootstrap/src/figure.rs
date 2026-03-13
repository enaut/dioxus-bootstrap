use dioxus::prelude::*;

/// Bootstrap Figure component — image with optional caption.
///
/// ```rust
/// rsx! {
///     Figure { src: "/img/photo.jpg", alt: "A photo",
///         caption: "A caption for the image.",
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct FigureProps {
    /// Image source URL.
    pub src: String,
    /// Alt text for the image.
    #[props(default)]
    pub alt: String,
    /// Caption text.
    #[props(default)]
    pub caption: String,
    /// Align caption (start, center, end).
    #[props(default)]
    pub caption_align: String,
    /// Make the image fluid (responsive).
    #[props(default = true)]
    pub fluid: bool,
    /// Add rounded corners to the image.
    #[props(default)]
    pub rounded: bool,
    /// Add thumbnail border to the image.
    #[props(default)]
    pub thumbnail: bool,
    /// Additional CSS classes for the figure.
    #[props(default)]
    pub class: String,
    /// Additional CSS classes for the image.
    #[props(default)]
    pub img_class: String,
}

#[component]
pub fn Figure(props: FigureProps) -> Element {
    let fig_class = if props.class.is_empty() {
        "figure".to_string()
    } else {
        format!("figure {}", props.class)
    };

    let mut img_classes = vec!["figure-img".to_string()];
    if props.fluid {
        img_classes.push("img-fluid".to_string());
    }
    if props.rounded {
        img_classes.push("rounded".to_string());
    }
    if props.thumbnail {
        img_classes.push("img-thumbnail".to_string());
    }
    if !props.img_class.is_empty() {
        img_classes.push(props.img_class.clone());
    }
    let img_class = img_classes.join(" ");

    let caption_class = if props.caption_align.is_empty() {
        "figure-caption".to_string()
    } else {
        format!("figure-caption text-{}", props.caption_align)
    };

    rsx! {
        figure { class: "{fig_class}",
            img {
                class: "{img_class}",
                src: "{props.src}",
                alt: "{props.alt}",
            }
            if !props.caption.is_empty() {
                figcaption { class: "{caption_class}", "{props.caption}" }
            }
        }
    }
}

/// Bootstrap responsive embed / aspect ratio wrapper.
///
/// ```rust
/// rsx! {
///     Ratio { aspect: "16x9",
///         iframe { src: "https://www.youtube.com/embed/..." }
///     }
/// }
/// ```
#[derive(Clone, PartialEq, Props)]
pub struct RatioProps {
    /// Aspect ratio: "1x1", "4x3", "16x9", "21x9".
    #[props(default = "16x9".to_string())]
    pub aspect: String,
    /// Additional CSS classes.
    #[props(default)]
    pub class: String,
    /// Child element (iframe, video, embed, etc.).
    pub children: Element,
}

#[component]
pub fn Ratio(props: RatioProps) -> Element {
    let full_class = if props.class.is_empty() {
        format!("ratio ratio-{}", props.aspect)
    } else {
        format!("ratio ratio-{} {}", props.aspect, props.class)
    };

    rsx! {
        div { class: "{full_class}",
            {props.children}
        }
    }
}
