// Declarar los submódulos
pub mod api;
pub mod admin;
pub mod public;
pub mod productos;

// Re-exportar las funciones principales para facilitar el uso
pub use api::create_api_router;
pub use admin::create_admin_router;
pub use public::create_public_router;
pub  use productos::create_productos_router;
