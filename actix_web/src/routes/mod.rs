pub mod application;
pub mod server;
pub mod extractors;
pub mod handlers;
pub mod errors;

pub use application::init_routes as application_routes;
pub use server::init_routes as server_routes;
pub use extractors::init_routes as extractor_routes;
pub use handlers::init_routes as handler_routes;
pub use errors::init_routes as error_routes;
