// =============================================================================
//! - Background Painter for the CroftSoft Animation Library
//!
//! # Metadata
//! - Copyright: &copy; 2023-2025 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-03
//! - Updated: 2025-01-14
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use com_croftsoft_lib_role::Painter;
use core::cell::RefCell;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

pub struct BackgroundPainter {
  canvas_height: f64,
  canvas_width: f64,
  context: Rc<RefCell<CanvasRenderingContext2d>>,
  fill_style: String,
}

impl BackgroundPainter {
  pub fn new(
    canvas_height: f64,
    canvas_width: f64,
    context: Rc<RefCell<CanvasRenderingContext2d>>,
    fill_style_str: &str,
  ) -> Self {
    let fill_style: String = fill_style_str.to_string();

    Self {
      canvas_height,
      canvas_width,
      context,
      fill_style,
    }
  }
}

impl Painter for BackgroundPainter {
  fn paint(&self) {
    let context = self.context.borrow();

    context.set_fill_style_str(&self.fill_style);

    context.fill_rect(0., 0., self.canvas_width, self.canvas_height);
  }
}
