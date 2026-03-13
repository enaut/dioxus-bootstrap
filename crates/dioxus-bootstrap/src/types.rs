use std::fmt;

/// Bootstrap contextual color variants.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Color {
    #[default]
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Primary => write!(f, "primary"),
            Color::Secondary => write!(f, "secondary"),
            Color::Success => write!(f, "success"),
            Color::Danger => write!(f, "danger"),
            Color::Warning => write!(f, "warning"),
            Color::Info => write!(f, "info"),
            Color::Light => write!(f, "light"),
            Color::Dark => write!(f, "dark"),
        }
    }
}

/// Bootstrap size variants.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Size {
    Sm,
    #[default]
    Md,
    Lg,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Size::Sm => write!(f, "sm"),
            Size::Md => write!(f, "md"),
            Size::Lg => write!(f, "lg"),
        }
    }
}

/// Bootstrap column span (1-12 or auto).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColumnSize {
    Auto,
    Span(u8),
}

impl fmt::Display for ColumnSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColumnSize::Auto => write!(f, "auto"),
            ColumnSize::Span(n) => write!(f, "{n}"),
        }
    }
}

/// Bootstrap navbar expand breakpoints.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum NavbarExpand {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Xxl,
    Always,
}

impl fmt::Display for NavbarExpand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NavbarExpand::Sm => write!(f, "navbar-expand-sm"),
            NavbarExpand::Md => write!(f, "navbar-expand-md"),
            NavbarExpand::Lg => write!(f, "navbar-expand-lg"),
            NavbarExpand::Xl => write!(f, "navbar-expand-xl"),
            NavbarExpand::Xxl => write!(f, "navbar-expand-xxl"),
            NavbarExpand::Always => write!(f, "navbar-expand"),
        }
    }
}

/// Bootstrap modal size.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ModalSize {
    Sm,
    #[default]
    Default,
    Lg,
    Xl,
}

/// Spinner style variant.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SpinnerStyle {
    #[default]
    Border,
    Grow,
}
