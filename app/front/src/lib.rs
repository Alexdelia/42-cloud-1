use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn on_keydown(event: web_sys::KeyboardEvent) {
    // Log the key that was pressed
    web_sys::console::log_1(&event.key().into());
}

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    let c = Closure::wrap(Box::new(on_keydown) as Box<dyn FnMut(_)>);
    window.add_event_listener_with_callback("keydown", c.as_ref().unchecked_ref())?;
    c.forget();

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
