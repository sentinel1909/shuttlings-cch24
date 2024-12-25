// src/lib/routes/day23.rs

// dependencies
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use axum_macros::debug_handler;
use html_escape::encode_text;
use http::StatusCode;
use std::collections::HashMap;

// Day 23 Handler - Task 2 /23/star
#[debug_handler]
#[tracing::instrument(name = "Day 23, Task 2 handler /23/star")]
pub async fn day23_task2() -> impl IntoResponse {
    let response_body = "<div id=\"star\" class=\"lit\"></div>";
    Html(response_body)
}

// Day 23 Handler - Task 3 /23/present/{color}
#[debug_handler]
#[tracing::instrument(name = "Day 23, Task 3 handler /23/present/{color}")]
pub async fn day23_task3(Path(color): Path<String>) -> impl IntoResponse {
    let mut color_map = HashMap::new();
    color_map.insert("red", "blue");
    color_map.insert("blue", "purple");
    color_map.insert("purple", "red");

    let color = encode_text(&color);
    if let Some(&next_color) = color_map.get(color.as_ref()) {
        let html = format!(
            r#"<div class="present {}" hx-get="/23/present/{}" hx-swap="outerHTML">
                    <div class="ribbon"></div>
                    <div class="ribbon"></div>
                    <div class="ribbon"></div>
                    <div class="ribbon"></div>
                </div>"#,
            color, next_color
        );
        Html(html).into_response()
    } else {
        (StatusCode::IM_A_TEAPOT).into_response()
    }
}

#[debug_handler]
#[tracing::instrument(name = "Day 23, Task 4 handler /23/ornament/{state}/{n}")]
pub async fn day23_task4(Path((state, n)): Path<(String, String)>) -> impl IntoResponse {
    let state = encode_text(&state);
    let n = encode_text(&n);
    
    if state != "on" && state != "off" {
        return (StatusCode::IM_A_TEAPOT).into_response();
    }

    let next_state = if state == "on" { "off" } else { "on" };

    let mut class = "ornament".to_string();
    if state == "on" {
        class.push(' ');
        class.push_str("on");
    }
    let html = format!(
        r#"<div class="{}" id="ornament{}" hx-trigger="load delay:2s once" hx-get="/23/ornament/{}/{}" hx-swap="outerHTML"></div>"#,
        class, n, next_state, n 
    );
    tracing::info!(html);
    (StatusCode::OK, Html(html)).into_response()
}
