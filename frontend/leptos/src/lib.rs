pub mod app;
pub mod component;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      // initializes logging using the `log` crate
      _ = console_log::init_with_level(log::Level::Debug);
      console_error_panic_hook::set_once();

      document().body().expect("no body exists").set_id("test");

      leptos::mount_to_body(move |cx| {
          view! { cx, <App/> }
      });
    }
}
}
