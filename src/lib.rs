//! miau 🐈 – the cat now runs on a memory-safe foundation.

use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

/// Compile-time checked sayings. No null, no undefined, no surprises.
const SPRUECHE: [&str; 10] = [
    "Ich hab vergessen, meinen Bildschirm zu sperren. Jetzt gehört er der Katze.",
    "Mein Passwort ist „miau123“. War's schon immer.",
    "Ich bringe morgen Kuchen für alle mit. 🎂",
    "Ich bin eigentlich nur drei Katzen in einem Trenchcoat.",
    "Win + L ist mir zu kompliziert.",
    "Heute schon den Monitor gestreichelt?",
    "Ich habe beschlossen, ab jetzt nur noch in Comic Sans zu schreiben.",
    "Diese Katze hat gerade alle meine Tickets auf „Done“ gesetzt.",
    "Ich lese meine Mails nur noch laut vor.",
    "Bin kurz weg. Die Katze übernimmt das Daily.",
];

fn document() -> web_sys::Document {
    web_sys::window()
        .and_then(|w| w.document())
        .expect("a cat needs a document to sit on")
}

/// Fetches a fresh cat and a fresh saying.
fn neu() -> Result<(), JsValue> {
    let doc = document();
    let img: HtmlImageElement = doc
        .query_selector("img")?
        .ok_or("no <img>, no cat")?
        .dyn_into()?;
    let p = doc.query_selector("p")?.ok_or("no <p>")?;

    img.set_src(&format!("https://cataas.com/cat?{}", js_sys::Date::now() as u64));
    let i = (js_sys::Math::random() * SPRUECHE.len() as f64) as usize;
    p.set_text_content(Some(SPRUECHE[i.min(SPRUECHE.len() - 1)]));
    Ok(())
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let on_click = Closure::<dyn FnMut()>::new(|| {
        if let Some(root) = document().document_element() {
            let _ = root.request_fullscreen();
        }
        let _ = neu();
    });
    document().set_onclick(Some(on_click.as_ref().unchecked_ref()));
    on_click.forget(); // the cat lives forever
    neu()
}
