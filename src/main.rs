use axum::extract::State;
use axum::{extract::Path, response::Html, routing::get, Router};
use minijinja::context;
use minijinja::Environment;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
struct Items {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize)]
struct Profile {
    full_name: String,
    items: Vec<Items>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub tpl_env: Environment<'static>,
}

#[tokio::main]
async fn main() {
    let mut env = minijinja::Environment::new();

    minijinja_embed::load_templates!(&mut env);
    // build our application with a single route
    let app = Router::new()
        .route("/", get(home))
        .route("/:profile_name", get(get_profile))
        .with_state(Arc::new(AppState { tpl_env: env }));

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    // run it with hyper on localhost:3000
    axum::serve(tcp_listener, app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> Html<&'static str> {
    Html("hello world")
}

async fn get_profile(
    Path(profile_name): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let orders_example = vec![
        Items {
            id: 1,
            name: "Article banana".into(),
        },
        Items {
            id: 2,
            name: "Article apple".into(),
        },
    ];
    let profile_example = Profile {
        full_name: profile_name,
        items: orders_example,
    };

    let template = state.tpl_env.get_template("profile_template.html").unwrap();
    let r = template
        .render(context! {profile => profile_example })
        .unwrap();
    Html(r)
}
