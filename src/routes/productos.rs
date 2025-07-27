use  salvo::prelude::*;


#[handler]
pub async fn get_productos() -> String {
    let _objetos = vec!["productos1", "producto2", "productos3"];
    format!("listado de productos desde /productos {}", _objetos.join(",").as_str())
}

pub fn create_productos_router() -> Router {
    Router::new().get(get_productos)
}