use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[wasm_bindgen]
pub fn hello(name: js_sys::JsString) -> js_sys::JsString {
    let name = name.as_string().unwrap_or_default();
    match name.as_str() {
        "" => "Hey there! It's good to have you here".into(),
        x => format!("Hi {}! Have a good time!", x),
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
