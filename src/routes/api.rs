use salvo::prelude::*;

// API Handlers
#[handler]
pub async fn get_users() -> &'static str {
    "Lista de usuarios desde /api/users"
}

#[handler]
pub async fn get_posts() -> &'static str {
    "Lista de posts desde /api/posts"
}

#[handler]
pub async fn create_user() -> &'static str {
    "Usuario creado desde POST /api/users"
}

#[handler]
pub async fn delete_user() -> &'static str {
    "Usuario eliminado desde DELETE /api/users"
}

// Función que retorna el router completo de API
pub fn create_api_router() -> Router {
    Router::new()
        .push(Router::with_path("users")
            .get(get_users)
            .post(create_user)
            .delete(delete_user))
        .push(Router::with_path("posts").get(get_posts))
}
