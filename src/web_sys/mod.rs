// =============================================================================
//! - web-sys functions for the CroftSoft Animation Library
//!
//! # Metadata
//! - Copyright: &copy; 2023-2025 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-07
//! - Updated: 2025-04-13
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

// TODO: see https://github.com/rustwasm/gloo

use ::anyhow::{Result, anyhow};
use ::futures::channel::mpsc::{UnboundedReceiver, unbounded};
use ::js_sys::Function;
use ::std::cell::Ref;
use ::std::{cell::RefCell, rc::Rc};
use ::wasm_bindgen::prelude::*;
use ::web_sys::{
  Document, DomRect, Element, Event, EventTarget, HtmlCanvasElement,
  HtmlElement, MouseEvent, Window, console, window,
};

/// A wasm_bindgen Closure for a request_animation_frame callback function
type CallbackClosure = Closure<dyn FnMut(f64)>;

pub trait LoopUpdater {
  fn update_loop(
    &mut self,
    timestamp: f64,
  ) -> bool;
}

pub fn add_change_handler(elem: HtmlElement) -> UnboundedReceiver<Event> {
  let (mut change_sender, change_receiver) = unbounded();
  let event_closure = move |event: Event| {
    let _result: Result<(), futures::channel::mpsc::SendError> =
      change_sender.start_send(event);
  };
  let event_closure_box: Box<dyn FnMut(Event)> = Box::new(event_closure);
  let on_change_closure: Closure<dyn FnMut(Event)> =
    Closure::wrap(event_closure_box);
  let closure_as_js_value_ref: &JsValue = on_change_closure.as_ref();
  let js_function_ref: &Function = closure_as_js_value_ref.unchecked_ref();
  let js_function_ref_option: Option<&Function> = Some(js_function_ref);
  elem.set_onchange(js_function_ref_option);
  on_change_closure.forget();
  change_receiver
}

pub fn add_change_handler_by_id(id: &str) -> Option<UnboundedReceiver<Event>> {
  let html_element = get_html_element_by_id(id);
  // TODO: return None if fails
  Some(add_change_handler(html_element))
}

pub fn add_click_handler(elem: HtmlElement) -> UnboundedReceiver<()> {
  let (mut click_sender, click_receiver) = unbounded();
  let on_click = Closure::wrap(Box::new(move || {
    let _result: Result<(), futures::channel::mpsc::SendError> =
      click_sender.start_send(());
  }) as Box<dyn FnMut()>);
  elem.set_onclick(Some(on_click.as_ref().unchecked_ref()));
  on_click.forget();
  click_receiver
}

pub fn add_click_handler_by_id(id: &str) -> Option<UnboundedReceiver<()>> {
  let html_element = get_html_element_by_id(id);
  // TODO: return None if fails
  Some(add_click_handler(html_element))
}

pub fn add_mouse_down_handler(
  elem: HtmlElement
) -> UnboundedReceiver<MouseEvent> {
  let (mut mouse_down_sender, mouse_down_receiver) = unbounded();
  let mouse_event_closure = move |mouse_event: MouseEvent| {
    let _result: Result<(), futures::channel::mpsc::SendError> =
      mouse_down_sender.start_send(mouse_event);
  };
  let mouse_event_closure_box: Box<dyn FnMut(MouseEvent)> =
    Box::new(mouse_event_closure);
  let on_mouse_down_closure: Closure<dyn FnMut(MouseEvent)> =
    Closure::wrap(mouse_event_closure_box);
  let closure_as_js_value_ref: &JsValue = on_mouse_down_closure.as_ref();
  let js_function_ref: &Function = closure_as_js_value_ref.unchecked_ref();
  let js_function_ref_option: Option<&Function> = Some(js_function_ref);
  elem.set_onmousedown(js_function_ref_option);
  on_mouse_down_closure.forget();
  mouse_down_receiver
}

pub fn add_mouse_down_handler_by_id(
  id: &str
) -> Option<UnboundedReceiver<MouseEvent>> {
  let html_element = get_html_element_by_id(id);
  // TODO: return None if fails
  Some(add_mouse_down_handler(html_element))
}

pub fn get_canvas_xy(mouse_event: &MouseEvent) -> (usize, usize) {
  let client_x: f64 = mouse_event.client_x() as f64;
  let client_y: f64 = mouse_event.client_y() as f64;
  let event_target: EventTarget = mouse_event.target().unwrap();
  let html_canvas_element: HtmlCanvasElement = event_target.dyn_into().unwrap();
  let dom_rect: DomRect = html_canvas_element.get_bounding_client_rect();
  let scale_x = html_canvas_element.width() as f64 / dom_rect.width();
  let scale_y = html_canvas_element.height() as f64 / dom_rect.height();
  let canvas_x: usize = ((client_x - dom_rect.left()) * scale_x) as usize;
  let canvas_y: usize = ((client_y - dom_rect.top()) * scale_y) as usize;
  (canvas_x, canvas_y)
}

pub fn get_html_canvas_element_by_id(
  canvas_element_id: &str
) -> HtmlCanvasElement {
  let document: Document = window().unwrap().document().unwrap();
  let element: Element = document.get_element_by_id(canvas_element_id).unwrap();
  element.dyn_into().unwrap()
}

pub fn get_html_element_by_id(id: &str) -> HtmlElement {
  let document: Document = window().unwrap().document().unwrap();
  let element: Element = document.get_element_by_id(id).unwrap();
  element.dyn_into().unwrap()
}

pub fn get_window() -> Result<Window> {
  web_sys::window().ok_or_else(|| anyhow!("No Window Found"))
}

pub fn log(message: &str) {
  console::log_1(&JsValue::from_str(message));
}

pub fn request_animation_frame(
  callback_closure_ref: &CallbackClosure
) -> Result<i32> {
  let window_result: Result<Window, anyhow::Error> = get_window();

  let window: Window = window_result?;

  let callback_shared_reference: &JsValue = callback_closure_ref.as_ref();

  let callback: &Function = callback_shared_reference.unchecked_ref();

  let request_id_result: Result<i32, JsValue> =
    window.request_animation_frame(callback);

  request_id_result.map_err(|err: JsValue| {
    anyhow!("Cannot request animation frame {:#?}", err)
  })
}

pub fn spawn_local_loop<L: LoopUpdater + 'static>(loop_updater: L) {
  wasm_bindgen_futures::spawn_local(async move {
    start_looping(loop_updater)
      .await
      .expect("loop start failed");
  });
}

pub async fn start_looping<L: LoopUpdater + 'static>(
  mut loop_updater: L
) -> Result<()> {
  let f: Rc<RefCell<Option<CallbackClosure>>> = Rc::new(RefCell::new(None));

  let g: Rc<RefCell<Option<CallbackClosure>>> = f.clone();

  let callback_function = move |timestamp: f64| {
    let stop: bool = loop_updater.update_loop(timestamp);

    if stop {
      return;
    }

    let f_borrow: Ref<'_, Option<CallbackClosure>> = f.borrow();

    let callback_closure_ref_option: Option<&CallbackClosure> =
      f_borrow.as_ref();

    let callback_closure_ref: &CallbackClosure =
      callback_closure_ref_option.unwrap();

    let _result: Result<i32> = request_animation_frame(callback_closure_ref);
  };

  let callback_function_box = Box::new(callback_function);

  let loop_closure: CallbackClosure = Closure::wrap(callback_function_box);

  *g.borrow_mut() = Some(loop_closure);

  let g_borrow: Ref<'_, Option<CallbackClosure>> = g.borrow();

  let loop_closure_ref_option: Option<&CallbackClosure> = g_borrow.as_ref();

  let callback: &CallbackClosure =
    loop_closure_ref_option.ok_or_else(|| anyhow!("loop failed"))?;

  let _request_id: i32 = request_animation_frame(callback)?;

  Ok(())
}
