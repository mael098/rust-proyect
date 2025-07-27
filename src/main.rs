use salvo::prelude::*;

// Handler for English greeting
#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

// Handler for Chinese greeting
#[handler]
async fn hello_zh() -> Result<&'static str, ()> {
    Ok("你好，世界！")
}

#[handler]
async fn cookie_handler(_req: &mut Request, res: &mut Response) {
    // Set a cookie in the response
    res.headers_mut().insert(
        "Set-Cookie",
        "my_cookie=my_value; Path=/; HttpOnly".parse().unwrap(),
    );

    // Respond with a simple message
    res.render(Text::Plain("Cookie has been set!"));
}

// API Handlers
#[handler]
async fn get_users() -> &'static str {
    "Lista de usuarios desde /api/users"
}

#[handler]
async fn get_posts() -> &'static str {
    "Lista de posts desde /api/posts"
}

#[handler]
async fn create_user() -> &'static str {
    "Usuario creado desde POST /api/users"
}

// Admin Handlers
#[handler]
async fn admin_dashboard() -> &'static str {
    "Panel de administración desde /admin/dashboard"
}

#[handler]
async fn admin_users() -> &'static str {
    "Gestión de usuarios desde /admin/users"
}

// Public Handlers
#[handler]
async fn about() -> &'static str {
    "Acerca de nosotros desde /public/about"
}

#[tokio::main]
async fn main() {
    // Initialize logging subsystem
    tracing_subscriber::fmt().init();

    // Bind server to port 5800
    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;

    // Crear sub-routers para organizar por "directorios"
    
    // API Router - todas las rutas empezarán con /api
    let api_router = Router::new()
        .push(Router::with_path("users").get(get_users).post(create_user))
        .push(Router::with_path("posts").get(get_posts));

    // Admin Router - todas las rutas empezarán con /admin
    let admin_router = Router::new()
        .push(Router::with_path("dashboard").get(admin_dashboard))
        .push(Router::with_path("users").get(admin_users));

    // Public Router - todas las rutas empezarán con /public
    let public_router = Router::new()
        .push(Router::with_path("about").get(about));

    // Router principal
    let router = Router::new()
        .get(hello)  // Ruta raíz
        .push(Router::with_path("你好").get(hello_zh))
        .push(Router::with_path("cookie").get(cookie_handler))
        .push(Router::with_path("api").push(api_router))      // Monta el API router en /api
        .push(Router::with_path("admin").push(admin_router))  // Monta el admin router en /admin
        .push(Router::with_path("public").push(public_router)); // Monta el public router en /public

    // Print router structure for debugging
    println!("{:?}", router);

    // Start serving requests
    Server::new(acceptor).serve(router).await;
}


