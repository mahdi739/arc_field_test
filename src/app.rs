use leptos::prelude::*;
use reactive_stores::{ArcField, ArcStore, Store};
#[derive(Store, Default)]
pub struct State {
  #[store(key:usize= |session| session.0)]
  sessions: Vec<(usize, String)>,
}

#[component]
pub fn App() -> impl IntoView {
  let selected_session: ArcStore<Option<(usize, String)>> = ArcStore::new(None);

  let state = ArcStore::new(State {
    sessions: vec![(0, "Item1".to_string()), (1, "Item2".to_string()), (2, "Item3".to_string())],
  });
  let selected_session_cloned = selected_session.clone();
  let selected_session_cloned2 = selected_session.clone();
  view! {
    <button on:click={
      let state_cloned = state.clone();
      let selected_session_cloned = selected_session.clone();
      let selected_session_cloned2 = selected_session.clone();
      move |_| {
        state_cloned
          .update(|state| {
            selected_session_cloned
              .get()
              .map(|selected_session| {
                let removing_index = state.sessions.iter().position(|session| session.0 == selected_session.0).unwrap();
                state.sessions.remove(removing_index);
              });
          });
        selected_session_cloned2.set(None);
      }
    }>"Delete selected item"</button>
    <For
      each={
        let state_cloned = state.clone();
        move || state_cloned.clone().sessions()
      }
      key=|item| item.get().0
      let(session)
    >
      <p on:click={
        let selected_session_cloned = selected_session.clone();
        move |_| {
          selected_session_cloned.set(Some(session.clone().get()));
        }
      }>
        {
          let session_cloned = session.clone();
          move || format!("{:?}", session_cloned.get())
        }
      </p>
    </For>
    <button on:click={move |_| {
      selected_session_cloned
        .update(|selected_session| {
          selected_session.as_mut().map(|f| f.1.push_str(" Hey!"));
        });
    }}>"Add Hey!"</button>
    <strong>"Selected Item: " {move || { format!("{:?}", selected_session_cloned2.get()) }}</strong>
  }
}
