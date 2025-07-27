fn index(){
    use salvo::prelude::*;
    use tracing::info;

    info!("Handling index route");

    let message = "Hello, world!";
    let response = format!("<h1>{}</h1>", message);
    
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(response)
        .into_response()
        .into()
}