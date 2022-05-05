use crate::{props::Props, react_bindings, VNode};
use wasm_bindgen::prelude::*;

/// Implement this trait on a struct to create a component with the struct as
/// props. The props struct has to be `'static`.
///
/// The props will be completely controlled by Rust, which makes rendering them
/// relatively simple in Rust. However, since the props struct cannot be
/// constructed in JS, these components cannot be exposed to JS. This means only
/// components written in Rust can render a `Component`.
///
/// **Warning:** Do not multiple components with the same name!
///
/// # Example
///
/// ```
/// struct Counter(i32);
///
/// impl Component for Counter {
///   fn name() -> &'static str {
///     "Counter"
///   }
///
///   fn render(&self) -> VNode {
///     h("div").build_with(children!["Counter: ", self.0])
///   }
/// }
/// ```
pub trait Component {
  /// The render function.
  ///
  /// **Do not** use this method in another render function. Instead, use one
  /// of the [`Component::into_vnode()`] methods.
  fn render(&self) -> VNode;

  /// Override this method to provide a [React key][key] when rendering.
  ///
  /// [key]: https://reactjs.org/docs/lists-and-keys.html
  fn key(&self) -> Option<String> {
    None
  }

  /// Returns a [`VNode`] of the component to be included in a
  /// [`Component::render()`] function.
  fn into_vnode(self) -> VNode
  where
    Self: Sized + 'static,
  {
    VNode(react_bindings::create_component(
      stringify!(Self),
      Props::new()
        .insert("key", self.key())
        .insert("component", ComponentWrapper(Box::new(self)))
        .into(),
    ))
  }
}

#[doc(hidden)]
#[wasm_bindgen(js_name = __WasmReact_ComponentWrapper)]
pub struct ComponentWrapper(Box<dyn Component>);

#[wasm_bindgen(js_class = __WasmReact_ComponentWrapper)]
impl ComponentWrapper {
  #[wasm_bindgen]
  pub fn render(props: &ComponentWrapper) -> JsValue {
    props.0.render().into()
  }
}
