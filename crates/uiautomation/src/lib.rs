pub mod actions;
pub mod controls;
pub mod core;
pub mod dialogs;
pub mod errors;
pub mod filters;
pub mod handlers;
pub mod inputs;
pub mod patterns;
pub mod processes;
pub mod types;
pub mod variants;

pub use self::errors::Error;
pub use self::errors::Result;

pub use self::core::UIAutomation;
pub use self::core::UIElement;
pub use self::core::UIMatcher;
pub use self::core::UITreeWalker;
