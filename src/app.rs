use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use anyhow::{Ok, Result};
use wasm_bindgen::JsValue;
use web_sys::{SerialOptions, SerialPort};
use web_sys::js_sys::JsString;
use futures::{stream, StreamExt};
use log::debug;

async fn serial() -> SerialPort {
    let promise = web_sys::window().unwrap().navigator().serial().request_port();
    let val = wasm_bindgen_futures::JsFuture::from(promise).await.expect("request_port failure");
    let val = SerialPort::from(val);
    let open_promise = val.open(&SerialOptions::new(115200));
    let val1 = wasm_bindgen_futures::JsFuture::from(open_promise).await
        .expect("Open failure");
    let rs = wasm_streams::ReadableStream::from_raw(val.readable());
    let mut st = rs.into_stream();
    loop {
        let c = st.next().await.
            expect("None")
            .expect("Result Next failure");
        let s = String::from_utf8(serde_wasm_bindgen::from_value(c).expect("to c failure")).expect("String failure");
        debug!("{}", s);
    }
    val
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/serial-mon.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let serial_req = create_action(|_: &String| {
        serial()
    });
    let port = serial_req.value();
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| 
    {
        serial_req.dispatch("test".to_string());
        set_count.update(|count| *count += 1);
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <p>{move || port().map(|p| <JsString as Into<String>>::into(p.to_string()))}</p>
        <p>{move || port().map(|p| 1
                               )}</p>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
