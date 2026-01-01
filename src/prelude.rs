//! Prelude module for dioxus-bulma
//!
//! This module re-exports the most commonly used items from dioxus-bulma,
//! allowing users to import everything they need with a single use statement:
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_bulma::prelude::*;
//! use dioxus_bulma::components::{Title, Subtitle}; // Import these explicitly to avoid conflicts with dioxus
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
    Button, Buttons, ButtonsAlignment,
    Content, Delete, Icon, Image, Notification, Progress,
    Table, TableContainer,
    Tag, Tags
    // Note: Title and Subtitle are NOT included to avoid conflicts with dioxus_document
};

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