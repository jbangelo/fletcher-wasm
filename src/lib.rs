mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{Event, FileReader, HtmlInputElement};

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn load_file(reader: FileReader) -> Vec<u8> {
    let buffer = js_sys::ArrayBuffer::from(reader.result().unwrap());
    js_sys::Uint8Array::new(&buffer).to_vec()
}

fn calc_fletcher16(data: Vec<u8>) -> u16 {
    fletcher::calc_fletcher16(&data)
}

fn calc_fletcher32(mut data: Vec<u8>) -> u32 {
    let remainder = data.len() % 2;
    for _ in 0..remainder {
        data.push(0);
    }

    let data: Vec<u16> = data
        .chunks_exact(2)
        .into_iter()
        .map(|bytes| u16::from_ne_bytes([bytes[0],bytes[1]]))
        .collect();

    fletcher::calc_fletcher32(&data)
}

fn calc_fletcher64(mut data: Vec<u8>) -> u64 {
    let remainder = data.len() % 4;
    for _ in 0..remainder {
        data.push(0);
    }

    let data: Vec<u32> = data
        .chunks_exact(4)
        .into_iter()
        .map(|bytes| u32::from_ne_bytes([bytes[0],bytes[1],bytes[2],bytes[3]]))
        .collect();

    fletcher::calc_fletcher64(&data)
}

fn on_click<T: std::fmt::UpperHex, F: Fn(Vec<u8>)->T + 'static>(f: F, f_name: String) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let input_elem = document.get_element_by_id("file_input").unwrap().dyn_into::<HtmlInputElement>().unwrap();
    let output_elem = document.get_element_by_id("output").unwrap();
    output_elem.set_inner_html("");

    let reader = web_sys::FileReader::new().unwrap();
    let on_load = Closure::wrap(Box::new(move |event: Event| {
        let reader = event.target().unwrap().dyn_into::<FileReader>().unwrap();
        let vec = load_file(reader);
        log!("Data length: {}", vec.len());
        let checksum = f(vec);
        log!("16-bit checksum: 0x{:04X}", checksum);
        output_elem.set_inner_html(&(output_elem.inner_html() + &format!("{} checksum: 0x{:04X}<br/>", f_name, checksum)));
    }) as Box<dyn FnMut(_)>);
    reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
    on_load.forget();

    let files = input_elem.files().unwrap();
    let file = files.get(0).unwrap();
    reader.read_as_array_buffer(&file).unwrap();
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    utils::set_panic_hook();

    log!("Hello from Rustlandia!");

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let button = document.get_element_by_id("calc16").unwrap();
    let f = Closure::wrap(Box::new(move |_event: Event| {
        on_click(calc_fletcher16, "Fletcher-16".to_string());
    }) as Box<dyn FnMut(_)>);
    button.add_event_listener_with_callback("click", f.as_ref().unchecked_ref()).unwrap();
    f.forget();

    let button = document.get_element_by_id("calc32").unwrap();
    let f = Closure::wrap(Box::new(move |_event: Event| {
        on_click(calc_fletcher32, "Fletcher-32".to_string());
    }) as Box<dyn FnMut(_)>);
    button.add_event_listener_with_callback("click", f.as_ref().unchecked_ref()).unwrap();
    f.forget();

    let button = document.get_element_by_id("calc64").unwrap();
    let f = Closure::wrap(Box::new(move |_event: Event| {
        on_click(calc_fletcher64, "Fletcher-64".to_string());
    }) as Box<dyn FnMut(_)>);
    button.add_event_listener_with_callback("click", f.as_ref().unchecked_ref()).unwrap();
    f.forget();

    Ok(())
}

