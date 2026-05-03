//! Prelude module for dioxus-bulma
//!
//! This module re-exports the most commonly used items from dioxus-bulma,
//! allowing users to import everything they need with a single use statement:
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_bulma::prelude::*;
//! ```
//!
//! ## Naming conflicts with Dioxus 0.7
//!
//! Dioxus 0.7 ships with a `Title` document component in `dioxus::prelude`
//! that mutates the page `<title>` element, and a similarly-named `Subtitle`
//! does not exist there. Bulma's `Title`/`Subtitle` are typography components
//! that render `<h1>`-`<h6>` tags. To avoid ambiguity when both preludes are
//! glob-imported, this prelude re-exports Bulma's typography components under
//! the names [`BulmaTitle`] and [`BulmaSubtitle`]. The original `Title` and
//! `Subtitle` names remain available via [`crate::components`] for users who
//! prefer them. This addresses
//! [issue #7](https://github.com/rexlunae/dioxus-bulma/issues/7).
//!
//! ```rust,ignore
//! use dioxus_bulma::prelude::*;
//!
//! rsx! {
//!     BulmaTitle { "My page" }
//!     BulmaSubtitle { "A subtitle" }
//! }
//! ```

// Re-export selective Dioxus essentials (avoiding conflicts with built-in components)
pub use dioxus::prelude::{component, Element, Props, rsx, EventHandler, MouseEvent, FormEvent, use_signal, use_effect, use_memo, use_callback};

// Theme system
pub use crate::theme::{BulmaTheme, BulmaColor, BulmaSize, BulmaProvider};

// Layout Components
pub use crate::components::{
    Container, Columns, Column, Section, Hero, HeroBody, HeroHead, HeroFoot,
    Level, LevelLeft, LevelRight, LevelItem,
    Media, MediaLeft, MediaContent, MediaRight,
    Tile
};

// Elements
pub use crate::components::{
    BulmaBox, Block,
    Button, Buttons, ButtonsAlignment,
    Content, Delete, Icon, Image, Notification, Progress,
    Table, TableContainer,
    Tag, Tags,
    TitleSize,
};

// Bulma's typography components, aliased to avoid colliding with Dioxus's
// document `Title` (and `Subtitle`, which only ships in `dioxus_document`).
pub use crate::components::{Title as BulmaTitle, Subtitle as BulmaSubtitle};

// Form Components
pub use crate::components::{
    Field, Label as FieldLabel, Help,
    Control, Input, InputType, Textarea, Select, Checkbox, Radio, File
};

// Components
pub use crate::components::{
    Breadcrumb, BreadcrumbItem,
    Card, CardHeader, CardHeaderTitle, CardContent, CardFooter, CardFooterItem,
    Dropdown, DropdownTrigger, DropdownMenu, DropdownItem, DropdownDivider,
    Menu, MenuLabel, MenuList, MenuItem,
    Message, MessageHeader, MessageBody,
    Modal, ModalCard, ModalCardHead, ModalCardBody, ModalCardFoot,
    Navbar, NavbarBrand, NavbarMenu, NavbarStart, NavbarEnd, NavbarItem,
    Pagination, PaginationPrevious, PaginationList, PaginationNext, PaginationLink, PaginationEllipsis,
    Panel, PanelHeading, PanelTabs, PanelBlock, PanelIcon,
    Tabs, Tab
};