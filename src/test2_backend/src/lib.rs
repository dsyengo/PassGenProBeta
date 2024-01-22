use wasm_bindgen::prelude::*;
use web_sys::{console, Document, Element, HtmlInputElement};

// Struct to represent the data structure
#[derive(Serialize, Deserialize)]
struct UserData {
    email_or_username: String,
    password: String,
}

// Function to save data to JSON file
#[wasm_bindgen]
pub fn save_password(website: &str, email: &str, password: &str) {
    let new_data = UserData {
        email_or_username: email.to_string(),
        password: password.to_string(),
    };

    // Fetch the document
    let document = Document::new().expect("document unavailable");

    // Serialize the data to JSON string
    let json_data = serde_json::to_string(&new_data).expect("serialization failed");

    // Retrieve existing data from local storage
    let existing_data = if let Some(data) = get_local_storage_item("data") {
        data
    } else {
        String::from("{}")
    };

    // Parse existing data into a JSON object
    let mut data: serde_json::Value =
        serde_json::from_str(&existing_data).expect("parsing failed");

    // Insert the new data
    data[website] = serde_json::from_str(&json_data).expect("parsing failed");

    // Save the updated data to local storage
    set_local_storage_item("data", &data.to_string());
}

// Function to find password based on website
#[wasm_bindgen]
pub fn find_password(website: &str) -> JsValue {
    // Retrieve data from local storage
    let data = if let Some(data) = get_local_storage_item("data") {
        data
    } else {
        String::from("{}")
    };

    // Parse data into a JSON object
    let data: serde_json::Value = serde_json::from_str(&data).expect("parsing failed");

    // Retrieve password data for the specified website
    if let Some(website_data) = data.get(website) {
        JsValue::from_serde(website_data).expect("serialization failed")
    } else {
        JsValue::NULL
    }
}

// Helper function to get an item from local storage
fn get_local_storage_item(key: &str) -> Option<String> {
    web_sys::window()?
        .local_storage()?
        .get_item(key)
        .ok()?
}

// Helper function to set an item in local storage
fn set_local_storage_item(key: &str, value: &str) {
    web_sys::window()
        .expect("window unavailable")
        .local_storage()
        .expect("local storage unavailable")
        .set_item(key, value)
        .expect("setting item failed");
}
