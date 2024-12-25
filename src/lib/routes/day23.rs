// src/lib/routes/day23.rs

// dependencies
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
};
use axum_macros::debug_handler;
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

  if let Some(&next_color) = color_map.get(color.as_str()) {
    let html = format!(
      r#"<div class="present {}" hx-get="/23/present/{}" hx-swap="outerHTML">
                    <div class="ribbon"></div>
                    <div class="ribbon"></div>
                    <div class="ribbon"></div>
                    <div class="ribbon"></div>
                </div>"#, color, next_color
    );
    Html(html).into_response()
  } else {
    (StatusCode::IM_A_TEAPOT).into_response()
  }
}
