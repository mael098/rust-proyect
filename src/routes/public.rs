use salvo::prelude::*;


// Public Handlers
#[handler]
pub async fn about() -> &'static str {
    "Acerca de nosotros desde /public/about"
}

#[handler]
pub async fn contact() -> &'static str {
    "Contáctanos desde /public/contact"
}

#[handler]
pub async fn help() -> &'static str {
    "Ayuda desde /public/help"
}

#[handler]
pub async fn terms() -> &'static str {
    "Términos y condiciones desde /public/terms"
}

// Función que retorna el router completo de Public
pub fn create_public_router() -> Router {
    Router::new()
        .push(Router::with_path("about").get(about))
        .push(Router::with_path("contact").get(contact))
        .push(Router::with_path("help").get(help))
        .push(Router::with_path("terms").get(terms))
}
