mod math;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn multiply(left: f64, right: f64) -> f64 {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = multiply(2.5, 2.3);
        assert_eq!(result, 5.75);
    }
}

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let result = multiply(3.0, 5.0);
    assert_eq!(result, 15.0);
}
