use js_sys::{Promise, Reflect};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use wasm_react::{
  export_components, h, hooks::use_state, Callback, Component, VNode,
};

pub struct App {
  name: Option<Rc<str>>,
}

impl TryFrom<JsValue> for App {
  type Error = JsValue;

  fn try_from(value: JsValue) -> Result<Self, Self::Error> {
    Ok(App {
      name: Reflect::get(&value, &"name".into())?
        .as_string()
        .map(|x| x.into()),
    })
  }
}

impl Component for App {
  fn render(&self) -> VNode {
    let callback_called = use_state(|| false);
    let callback_called_value = callback_called.value();

    (
      DelayedButton {
        callback: {
          let mut callback_called = callback_called.clone();
          move || callback_called.set(|_| true)
        },
      }
      .build(),
      VNode::from(match *callback_called_value {
        true => "Callback called",
        false => "Callback not called",
      }),
    )
      .into()
  }
}

struct DelayedButton<T: FnMut() + Clone> {
  callback: T,
}

impl<T: FnMut() + Clone + 'static> Component for DelayedButton<T> {
  fn render(&self) -> VNode {
    let mut callback = self.callback.clone();
    h!(button)
      .on_click(&Callback::new(move |_| {
        // This works
        callback();

        // This doesn't work
        spawn_local(async move {
          JsFuture::from(Promise::resolve(&JsValue::UNDEFINED))
            .await
            .unwrap();
          callback();
        });
      }))
      .build("Click me")
  }
}

export_components! { App }
