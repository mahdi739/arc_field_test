use leptos::wasm_bindgen;
use leptos::web_sys;
use leptos::{prelude::*, task::spawn_local};
use reactive_stores::StoreField;
use reactive_stores::{ArcField, ArcStore, Field, OptionStoreExt, Store};
use std::ops::Deref;
use std::ops::DerefMut;
use std::time::Duration;
use wasm_bindgen_futures::JsFuture;
#[derive(Store, Default)]
pub struct State {
  #[store(key:usize= |session| session.0)]
  sessions: Vec<(usize, String)>,
  selected_session: Option<Field<(usize, String)>>,
}
pub async fn async_sleep(ms: i32) {
  // Convert milliseconds to seconds for Duration
  // let duration = Duration::from_millis(ms as u64);

  // Use js_sys::Promise for setting up a timer
  let promise = web_sys::js_sys::Promise::new(&mut |resolve, _| {
    // Call JavaScript's setTimeout
    let window = leptos::web_sys::window().unwrap();
    window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
  });

  // Await the promise
  JsFuture::from(promise).await.unwrap();
}

#[component]
pub fn App() -> impl IntoView {
  let state = Store::new(State {
    sessions: vec![(0, "ItemA".to_string()), (1, "ItemB".to_string()), (2, "ItemC".to_string())],
    selected_session: None,
  });
  view! {
    <button on:click=move |_| {
      state
        .selected_session()
        .get()
        .map(|selected_session| {
          let selected = selected_session.read().0;
          let removing_index = state
            .sessions()
            .read()
            .iter()
            .position(|session| session.0 == selected)
            .unwrap();
          state.sessions().write().remove(removing_index);
        });
      state.selected_session().set(None);
    }>"Delete selected item"</button>
    <For each=move || state.sessions() key=|item| item.get().0 let(session)>
      <p on:click=move |_| {
        state.selected_session().set(Some(session.into()));
      }>{move || format!("{:?}", session.get())}</p>
    </For>
    <button on:click=move |_| {
      if let Some(selected_session) = state.selected_session().get() {
        let selected_session = selected_session.read().0;
        for session in state.sessions() {
          if session.read().0 == selected_session {
            session.write().deref_mut().1.push_str(" Hey!");
          }
        }
      }
    }>"Add Hey!"</button>
    <strong>
      "Selected Item: " {move || { format!("{:?}", state.selected_session().get().get()) }}
    </strong>
  }
}
