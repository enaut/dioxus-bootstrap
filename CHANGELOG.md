# Changelog

All notable changes to dioxus-bootstrap-css are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [0.2.5]

- Toast supports headerless mode with close button (Bootstrap 5.3 `d-flex` pattern): omit `title` and set `show_close: true`
- Toast `on_dismiss` callback fires when the toast is dismissed

## [0.2.4]

- Alert `on_dismiss` callback fires when a dismissible alert is closed

## [0.2.3]

- Button `target` prop (e.g., `"_blank"` for new tab)
- Button `download` prop (triggers file download when used with `href`)

## [0.2.2]

- Button `href` prop: renders `<a>` instead of `<button>` for link-button pattern

## [0.2.1]

- All components extend `GlobalAttributes` — Card, Table, Nav, Modal, Grid, Form, and others accept any standard HTML attribute (`id`, `title`, `aria-*`, `data-*`, etc.)

## [0.2.0]

- Button extends `GlobalAttributes`: accepts any HTML attribute directly

## [0.1.9]

- Bug fixes

## [0.1.8]

- Card styling improvements

## [0.1.7]

- Card `header_class`, `body_class`, `footer_class` props for fine-grained section styling

## [0.1.4 – 0.1.6]

- Initial release with core Bootstrap 5.3 components: Button, Card, Alert, Badge, Table, Modal, Dropdown, Tabs, Accordion, Collapse, Nav, Navbar, Form controls, Grid, Icon, Toast, Carousel, Tooltip, Popover, Offcanvas, Scrollspy, Progress, Spinner, Placeholder, ListGroup, Pagination, Breadcrumb, Figure, Ratio, ThemeProvider, BootstrapHead
