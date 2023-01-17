//! src/routes/admin/mod.rs

mod dashboard;
mod logout;
mod password;

pub use dashboard::admin_dashboard;
pub use logout::*;
pub use password::*;
