use leptos::prelude::*;
use reactive_stores::{Field, Store, StoreFieldIterator};
#[derive(Store, Default, Clone)]
pub struct State {
  #[store(key:usize = |session| session.0)]
  sessions: Vec<Session>,
}
#[derive(Store, Default, PartialEq, Eq, Hash, Clone)]
pub struct Session(usize, String);

#[component]
pub fn App() -> impl IntoView {
  let selected_session: RwSignal<Option<Field<Session>>> = RwSignal::new(None);
  
  let state = Store::new(State {
    sessions: vec![
      Session(0, "Item1".to_string()),
      Session(1, "Item2".to_string()),
      Session(2, "Item3".to_string()),
    ],
  });

  view! {
    <button on:click=move |_| {
      selected_session
        .get()
        .map(|ss| {
          let removing_index = state
            .sessions()
            .iter_unkeyed()
            .position(|session| session.get() == ss.get())
            .unwrap();
          let is_same_session = state.sessions().at_unkeyed(removing_index).get() == ss.get();
          state.sessions().write().remove(removing_index);
          if is_same_session {
            selected_session.set(None);
          }
        });
    }>"Delete selected item"</button>
    <For each=move || state.sessions() key=|item| item.get() let(session)>
      <p on:click=move |_| {
        selected_session.set(Some(session.into()));
      }>{move || session.field1().get()}</p>
    </For>

    <strong>
      "Selected Item: "
      {move || {
        selected_session
          .try_get()
          .flatten()
          .map(|item| {
            view! { {move || item.field1().get()} }
          })
      }}
    </strong>
  }
}
