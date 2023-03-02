// =============================================================================
//! - Frame Rate Updater
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-03-01
//! - Updated: 2023-03-01
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::FrameRater;
use com_croftsoft_lib_role::Updater;
use core::cell::{Ref, RefCell};
use std::rc::Rc;

pub trait FrameRaterUpdaterInputs {
  fn get_frame_rate_display_change_requested(&self) -> Option<bool>;
  fn get_reset_requested(&self) -> bool;
  fn get_time_to_update(&self) -> bool;
  fn get_update_period_millis_changed(&self) -> Option<f64>;
  fn get_update_time_millis(&self) -> f64;
}

pub struct FrameRaterUpdater {
  frame_rate_display: bool,
  frame_rater: Rc<RefCell<dyn FrameRater>>,
  inputs: Rc<RefCell<dyn FrameRaterUpdaterInputs>>,
}

impl FrameRaterUpdater {
  pub fn new(
    frame_rate_display: bool,
    frame_rater: Rc<RefCell<dyn FrameRater>>,
    inputs: Rc<RefCell<dyn FrameRaterUpdaterInputs>>,
  ) -> Self {
    Self {
      frame_rate_display,
      frame_rater,
      inputs,
    }
  }
}

impl Updater for FrameRaterUpdater {
  fn update(&mut self) {
    let inputs: Ref<dyn FrameRaterUpdaterInputs> = self.inputs.borrow();
    if let Some(frame_rate_display) =
      inputs.get_frame_rate_display_change_requested()
    {
      self.frame_rate_display = frame_rate_display;
      if frame_rate_display {
        self.frame_rater.borrow_mut().clear();
      }
    }
    if let Some(update_period_millis) =
      inputs.get_update_period_millis_changed()
    {
      self
        .frame_rater
        .borrow_mut()
        .update_frame_sample_size(update_period_millis);
    }
    if inputs.get_reset_requested() {
      self.frame_rater.borrow_mut().clear();
      return;
    }
    if self.frame_rate_display && inputs.get_time_to_update() {
      self
        .frame_rater
        .borrow_mut()
        .sample(self.inputs.borrow().get_update_time_millis());
    }
  }
}
