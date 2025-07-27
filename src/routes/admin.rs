use salvo::prelude::*;

// Admin Handlers
#[handler]
pub async fn admin_dashboard() -> &'static str {
    "Panel de administración desde /admin/dashboard"
}

#[handler]
pub async fn admin_users() -> &'static str {
    "Gestión de usuarios desde /admin/users"
}

#[handler]
pub async fn admin_settings() -> &'static str {
    "Configuraciones desde /admin/settings"
}

#[handler]
pub async fn admin_stats() -> &'static str {
    "Estadísticas desde /admin/stats"
}

// Función que retorna el router completo de Admin
pub fn create_admin_router() -> Router {
    Router::new()
        .push(Router::with_path("dashboard").get(admin_dashboard))
        .push(Router::with_path("users").get(admin_users))
        .push(Router::with_path("settings").get(admin_settings))
        .push(Router::with_path("stats").get(admin_stats))
}
