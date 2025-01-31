use leptos::prelude::*;
use reactive_stores::{ArcField, ArcStore, Store};
#[derive(Store, Default)]
pub struct State {
  #[store(key:String = |session| session.clone())]
  sessions: Vec<String>,
}

#[component]
pub fn App() -> impl IntoView {
  let selected_session: ArcStore<Option<String>> = ArcStore::new(None);

  let state = ArcStore::new(State {
    sessions: vec!["Item1".to_string(), "Item2".to_string(), "Item3".to_string()],
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
                let removing_index = state.sessions.iter().position(|session| *session == selected_session).unwrap();
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
      key=|item| item.get()
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
          move || session_cloned.get()
        }
      </p>
    </For>
    <button on:click={move |_| {
      selected_session_cloned
        .update(|selected_session| {
          selected_session.as_mut().map(|f| f.push_str(" Hey!"));
        });
    }}>"Add Hey!"</button>
    <strong>"Selected Item: " {move || { format!("{:?}", selected_session_cloned2.get()) }}</strong>
  }
}
