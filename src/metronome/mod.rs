// =============================================================================
//! - Metronome trait
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-21
//! - Updated: 2023-02-22
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub mod delta;
pub mod simple;
pub mod updater;

pub trait Metronome {
  fn reset(
    &mut self,
    current_time_millis: f64,
  );

  fn set_period_millis(
    &mut self,
    period_millis: f64,
  );

  fn set_time_millis_next_tick(
    &mut self,
    time_millis_next_tick: f64,
  );

  fn tick(
    &mut self,
    current_time_millis: f64,
  ) -> bool;
}
