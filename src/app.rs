use leptos::prelude::*;
use reactive_stores::{Field, OptionStoreExt, Store, StoreFieldIterator};
use uuid::Uuid;
#[derive(Store, Default, Clone)]
pub struct State {
  #[store(key:Uuid = |session| session.id)]
  sessions: Vec<Session>,
  selected_session: Option<Field<Session>>,
}
#[derive(Store, Default, PartialEq, Eq, Hash, Clone)]
pub struct Session {
  id: Uuid,
  label: String,
}

#[component]
pub fn App() -> impl IntoView {
  let state = Store::new(State {
    sessions: vec![
      Session { id: Uuid::new_v4(), label: "Item1".to_string() },
      Session { id: Uuid::new_v4(), label: "Item2".to_string() },
      Session { id: Uuid::new_v4(), label: "Item3".to_string() },
    ],
    selected_session: None,
  });
  view! {
    <button on:click=move |_| {
      state
        .selected_session()
        .map(|selected_session| {
          let removing_index = state
            .sessions()
            .iter_unkeyed()
            .position(|session| session.get() == selected_session.get().get())
            .unwrap();
          let is_same_session = state.sessions().at_unkeyed(removing_index).get()
            == selected_session.get().get();
          state.sessions().write().remove(removing_index);
          if is_same_session {
            state.selected_session().set(None);
          }
        });
    }>"Delete selected item"</button>
    <ForEnumerate each=move || state.sessions() key=|item| item.get() let(index, session)>

      <p on:click=move |_| {
        state.selected_session().set(Some(session.into()));
      }>
        <button
          on:click=move |_| {
            let is_session_selected = state
              .selected_session()
              .read()
              .map(|f| *f.read() == *session.read())
              .unwrap_or(false);
            state.sessions().write().remove(index.get());
            if is_session_selected {
              state.selected_session().set(state.sessions().into_iter().next().map(Into::into));
            }
          }
          class="fa fa-trash delete"
        />
        {move || session.get().label}
      </p>
    </ForEnumerate>

    <strong>
      "Selected Item: "
      {move || {
        state
          .selected_session()
          .try_get()
          .flatten()
          .map(|item| {
            view! { <button disabled=move || item.label().get().len() < 1>Hey</button> }
          })
      }}
    </strong>
  }
}
