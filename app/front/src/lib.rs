use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn on_keydown(event: web_sys::KeyboardEvent) {
    web_sys::console::log_1(&event.key().into());

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let field = document
        .get_element_by_id("typing")
        .expect("cannot find element with id 'typing'");

    field.set_text_content(Some(&event.key()));
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    // let body = document.body().expect("document should have a body");

    let c = Closure::wrap(Box::new(on_keydown) as Box<dyn FnMut(_)>);
    window.add_event_listener_with_callback("keydown", c.as_ref().unchecked_ref())?;
    c.forget();

    let field = document
        .get_element_by_id("typing")
        .expect("cannot find element with id 'typing'");
    field.set_text_content(Some("Press any key"));

    Ok(())
}
