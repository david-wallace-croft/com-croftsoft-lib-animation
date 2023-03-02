// =============================================================================
//! - Frame Rater
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Created: 2023-02-25
//! - Updated: 2023-03-01
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

pub mod simple;

pub trait FrameRater {
  fn clear(&mut self);

  fn get_frames_per_second_sampled(&self) -> f64;

  fn sample(
    &mut self,
    update_time_millis: f64,
  ) -> bool;

  fn update_frame_sample_size(
    &mut self,
    frame_period_millis: f64,
  );
}
