use serde::{Deserialize, Serialize};

/// A struct that relates a server, with the channel it sends welcome images if any.
pub struct WelcomesIn {
  /// A Discord guild's ID.
  pub from: u64,
  /// The previously said channel.
  pub to: u64,
  /// Defines when an image should be sent.
  pub when: Vec<WelcomesWhen>,
}

#[derive(Deserialize, Serialize)]
/// Defines when an image should be sent.
pub enum WelcomesWhen {
  /// If present an image will be sent as welcome.
  In,
  /// If present an image will be sent as goodbye.
  Out,
}
