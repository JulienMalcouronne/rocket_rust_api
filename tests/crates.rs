use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_crates() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);
    let b_crate = common::create_test_crate(&client, &rustacean);

    let response = client
        .get(format!("{}/crates", common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&a_crate));
    assert!(json.as_array().unwrap().contains(&b_crate));

    common::delete_test_crate(&client, a_crate);
    common::delete_test_crate(&client, b_crate);
    common::delete_test_rustacean(&client, rustacean)
}
#[test]

fn test_create_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .post(format!("{}/crates", common::APP_HOST))
        .json(&json!({
          "rustacean_id": rustacean["id"],
          "code": "foo",
          "name": "Foo crate",
          "version": "O.1",
          "description": "foo crate description"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(
        a_crate,
        json!({
          "id": a_crate["id"],
          "rustacean_id": rustacean["id"],
          "code": "foo",
          "name": "Foo crate",
          "version": "O.1",
          "description": "foo crate description",
          "created_at": a_crate["created_at"],
        })
    );
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client
        .get(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let a_crate: Value = response.json().unwrap();
    assert_eq!(
        a_crate,
        json!({
          "id": a_crate["id"],
          "rustacean_id": rustacean["id"],
          "code": "foo",
          "name": "Foo crate",
          "version": "O.1",
          "description": "foo crate description",
          "created_at": a_crate["created_at"],
        })
    );

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client
        .put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .json(&json!({
          "rustacean_id": rustacean["id"],
          "code": "fooz",
          "name": "Fooz crate",
          "version": "O.2",
          "description": "fooz crate description",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let a_crate: Value = response.json().unwrap();
    assert_eq!(
        a_crate,
        json!({
          "id": a_crate["id"],
          "rustacean_id": rustacean["id"],
          "code": "fooz",
          "name": "Fooz crate",
          "version": "O.2",
          "description": "fooz crate description",
          "created_at": a_crate["created_at"],
        })
    );

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client
        .delete(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    common::delete_test_rustacean(&client, rustacean);
}
