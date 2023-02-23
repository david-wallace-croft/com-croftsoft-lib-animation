// =============================================================================
//! - An alternative implementation of the Metronome trait
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-22
//! - Updated: 2023-02-22
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

use super::Metronome;

pub struct DeltaMetronome {
  pub period_millis: f64,
  pub time_millis_next_tick: f64,
}

impl Metronome for DeltaMetronome {
  fn reset(
    &mut self,
    current_time_millis: f64,
  ) {
    self.time_millis_next_tick = current_time_millis + self.period_millis
  }

  fn set_period_millis(
    &mut self,
    period_millis: f64,
  ) {
    self.period_millis = period_millis;
  }

  fn set_time_millis_next_tick(
    &mut self,
    time_millis_next_tick: f64,
  ) {
    self.time_millis_next_tick = time_millis_next_tick;
  }

  fn tick(
    &mut self,
    current_time_millis: f64,
  ) -> bool {
    let overshoot_millis = current_time_millis - self.time_millis_next_tick;
    if overshoot_millis < 0. {
      return false;
    }
    let mut delta_millis = self.period_millis - overshoot_millis;
    if delta_millis < 0. {
      delta_millis = self.period_millis;
    }
    self.time_millis_next_tick = current_time_millis + delta_millis;
    true
  }
}
