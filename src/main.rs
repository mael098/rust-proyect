use salvo::prelude::*;

// Importar los módulos de rutas
mod routes;
use routes::{create_api_router, create_admin_router, create_public_router, create_productos_router};

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

#[tokio::main]
async fn main() {
    // Initialize logging subsystem
    tracing_subscriber::fmt().init();

    // Bind server to port 5800
    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;

    // Crear sub-routers usando las funciones de los módulos
    let api_router = create_api_router();
    let admin_router = create_admin_router();
    let public_router = create_public_router();
    let productos_router = create_productos_router();

    // Router principal
    let router = Router::new()
        .get(hello)  // Ruta raíz
        .push(Router::with_path("你好").get(hello_zh))
        .push(Router::with_path("cookie").get(cookie_handler))
        .push(Router::with_path("api").push(api_router))      // Monta el API router en /api
        .push(Router::with_path("admin").push(admin_router))  // Monta el admin router en /admin
        .push(Router::with_path("public").push(public_router)) // Monta el public router en /public
        .push(Router::with_path("productos").push(productos_router)); // Monta el productos router en /productos

    // Print router structure for debugging
    println!("{:?}", router);

    // Start serving requests
    Server::new(acceptor).serve(router).await;
}


