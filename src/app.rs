use leptos::prelude::*;
use reactive_stores::{ArcField, ArcStore, Store};
#[derive(Store, Default)]
pub struct State {
  #[store(key:String = |session| session.clone())]
  sessions: Vec<String>,
  #[store(skip)]
  selected_session: Option<ArcField<String>>,
}

#[component]
pub fn App() -> impl IntoView {
  let state = ArcStore::new(State {
    sessions: vec!["Item1".to_string(), "Item2".to_string(), "Item3".to_string()],
    selected_session: None,
  });
  let state_cloned = state.clone();
  view! {
    <button on:click={
      let state_cloned = state.clone();
      move |_| {
        state_cloned
          .update(|state| {
            state
              .selected_session
              .clone()
              .map(|selected_session| {
                let removing_index = state
                  .sessions
                  .iter()
                  .position(|session| *session == selected_session.get())
                  .unwrap();
                state.sessions.remove(removing_index);
                state.selected_session = None;
              });
          })
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
        let state_cloned = state.clone();
        move |_| {
          state_cloned.update(|state| state.selected_session = Some(session.clone().into()));
        }
      }>
        {
          let session_cloned = session.clone();
          move || session_cloned.get()
        }
      </p>
    </For>

    <strong>
      "Selected Item: "
      {move || {
        format!(
          "{:?}",
          state_cloned
            .with(|state| {
              state.selected_session.clone().map(|selected_Session| selected_Session.get())
            }),
        )
      }}
    </strong>
  }
}
