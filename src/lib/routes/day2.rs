// src/lib/routes/day2.rs

// dependencies
use axum::{extract::Query, response::IntoResponse};
use axum_macros::{self, debug_handler};
use serde::Deserialize;
use std::net::Ipv6Addr;
use std::str::FromStr;

// struct type to represent the Query parameters for Day 2, Task 1
#[derive(Deserialize)]
pub struct EncryptionParameters {
    pub from: String,
    pub key: String,
}

// struct type to represent the Query parameters for Day2, Task 2
#[derive(Deserialize)]
pub struct DecryptionParameters {
    pub from: String,
    pub to: String,
}

// Day 2, Task 1 handler
#[debug_handler]
#[tracing::instrument(name = "Day 2, Task 1" skip(params))]
pub async fn day2_task1(params: Query<EncryptionParameters>) -> impl IntoResponse {
    let input = params.0;
    let from = input.from;
    let key = input.key;

    let from_octets: Vec<_> = from.split(".").map(|o| o.parse::<u8>().unwrap()).collect();

    let key_octets: Vec<_> = key.split(".").map(|k| k.parse::<u8>().unwrap()).collect();
    let mut dest: Vec<u8> = Vec::new();

    for (i, item) in key_octets.iter().enumerate() {
        let (sum, _) = item.overflowing_add(from_octets[i]);
        dest.push(sum);
    }

    let mut dest_str = String::new();

    for (i, item) in dest.iter().enumerate() {
        dest_str.push_str(&item.to_string());

        if i < dest.len() - 1 {
            dest_str.push('.');
        }
    }

    dest_str
}

// Day 2, Task 2 handler
#[debug_handler]
#[tracing::instrument(name = "Day 2, Task 2", skip(params))]
pub async fn day2_task2(params: Query<DecryptionParameters>) -> impl IntoResponse {
    let input = params.0;
    let from = input.from;
    let to = input.to;

    let from_octets: Vec<_> = from.split(".").map(|o| o.parse::<u8>().unwrap()).collect();

    let to_octets: Vec<_> = to.split(".").map(|k| k.parse::<u8>().unwrap()).collect();
    let mut key: Vec<u8> = Vec::new();

    for (i, item) in to_octets.iter().enumerate() {
        let (diff, _) = item.overflowing_sub(from_octets[i]);
        key.push(diff);
    }

    let mut key_str = String::new();

    for (i, item) in key.iter().enumerate() {
        key_str.push_str(&item.to_string());

        if i < key.len() - 1 {
            key_str.push('.');
        }
    }

    key_str
}

// Day 2, Task 3 Encrypt handler
#[debug_handler]
#[tracing::instrument(name = "Day 2, Task3 - Encrypt ", skip(params))]
pub async fn day2_task3_encrypt(params: Query<EncryptionParameters>) -> impl IntoResponse {
    let from = params.0.from;
    let key = params.0.key;

    let from_ipv6 = Ipv6Addr::from_str(&from).unwrap().to_bits();
    let key_ipv6 = Ipv6Addr::from_str(&key).unwrap().to_bits();

    let dest_bits = from_ipv6 ^ key_ipv6;

    let dest = Ipv6Addr::from_bits(dest_bits);

    dest.to_string()
}

// Day 2, Task 3 Decrypt handler
#[debug_handler]
#[tracing::instrument(name = "Day 2, Task3 - Decrypt", skip(params))]
pub async fn day2_task3_decrypt(params: Query<DecryptionParameters>) -> impl IntoResponse {
    let from = params.0.from;
    let to = params.0.to;

    let from_ipv6 = Ipv6Addr::from_str(&from).unwrap().to_bits();
    let to_ipv6 = Ipv6Addr::from_str(&to).unwrap().to_bits();

    let key_bits = from_ipv6 ^ to_ipv6;

    let key = Ipv6Addr::from(key_bits);

    key.to_string()
}
