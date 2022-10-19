use axum::{extract::Path, response::Html, routing::get, Router};
use minijinja::render;
use serde::Serialize;

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

const HOME: &'static str = r#"
<!doctype html>

<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>A Basic HTML5 Template</title>
  <meta name="description" content="A simple HTML5 Template for new projects.">
  <meta name="author" content="Woile">
</head>

<body>
    <h1>Welcome to this example website!</h1>
</body>
</html>
"#;

const PROFILE_TEMPLATE: &'static str = r#"
<!doctype html>

<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>A Basic HTML5 Template</title>
  <meta name="description" content="A simple HTML5 Template for new projects.">
  <meta name="author" content="Woile">
</head>

<body>
    <h1>Profile of {{ profile.full_name|title }}</h1>
    <p>This is a template example just to show some functionality</p>
    <h2>Items</h3>
    <ul>
        {% for item in profile.items %}
        <li>{{ item.name }} ({{ item.id }})</li>
        {% endfor %}
    <ul>
</body>
</html>
"#;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                Html(HOME)
            }),
        )
        .route(
            "/:profile_name",
            get(|Path(profile_name): Path<String>| async {
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
                let r = render!(PROFILE_TEMPLATE, profile => profile_example );
                Html(r)
            }),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
