use leptos::prelude::*;
use reactive_stores::{ArcField, ArcStore, Field, Store};
#[derive(Store, Default, Clone)]
pub struct State {
  #[store(key:usize= |session| session.id)]
  sessions: Vec<Session>,
}
#[derive(Store, Default, Clone)]
pub struct Session {
  id: usize,
  label: String,
}
#[component]
pub fn App() -> impl IntoView {
  let selected_session: RwSignal<Option<Field<Session>>> = RwSignal::new(None);

  let state = Store::new(State {
    sessions: vec![
      Session { id: 0, label: "ItemA".to_string() },
      Session { id: 1, label: "ItemB".to_string() },
      Session { id: 2, label: "ItemC".to_string() },
    ],
  });
  view! {
    <button on:click=move |_| {
      selected_session
        .get()
        .map(|selected_session| {
          let selected = selected_session.read().id;
          let removing_index = state
            .sessions()
            .read()
            .iter()
            .position(|session| session.id == selected)
            .unwrap();
          state.sessions().write().remove(removing_index);
        });
      selected_session.set(None);
    }>"Delete selected item"</button>
    <For each=move || state.sessions() key=|item| item.id().get() let(session)>
      <p on:click=move |_| {
        selected_session.set(Some(session.into()));
      }>{move || format!("{:?}", session.label().get())}</p>
    </For>
    <button on:click=move |_| {
      if let Some(selected_session) = selected_session.get() {
        let selected_session = selected_session.read().id;
        for session in state.sessions() {
          if session.read().id == selected_session {
            session.write().label.push_str(" Hey!");
          }
        }
      }
    }>"Add Hey!"</button>
    <strong>
      "Selected Item: "
      {move || { format!("{:?}", selected_session.get().map(|f| f.label()).get()) }}
    </strong>
    {move || {
      selected_session
        .get()
        .map(|ss| {
          view! { <button disabled=move || ss.get().label.len() < 1>Test</button> }
        })
    }}
  }
}
