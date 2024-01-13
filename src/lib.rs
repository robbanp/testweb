use dotenvy_macro::dotenv;
use dotenvy::dotenv;
use sea_orm::Database;
use entity;
mod routes;

pub async fn run() {
    dotenv().ok();
    let conn_str = dotenv!("DATABASE_URL");
    let db = Database::connect(conn_str).await.unwrap();

//    let e = entity::Post {id: 1, title: "hej".to_owned(), text: "lol".to_owned()};

    let app = routes::create_routes(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

