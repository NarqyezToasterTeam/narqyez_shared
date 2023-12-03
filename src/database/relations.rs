use serde::{Deserialize, Serialize};
use serenity::all::{Context, Member};

/// Define a relations where
///
/// - **from** *(in)* is a Discord user.
/// - **to** is the Guild's he is allowed to update our bot settings.
#[derive(Deserialize, Serialize)]
pub struct IsAdminOn {
  #[serde(rename = "in")]
  /// A Discord user's ID.
  pub from: u64,
  /// The guild the user allowed to update our bot settings.
  pub to: u64,
}

/// Maintains user permissions for each server where both the user and our bot coexist,
/// enhancing security through the integration of a permission system in the database.
#[derive(Deserialize, Serialize)]
pub struct PermissionsOn {
  /// A Discord user's ID.
  pub from: u64,
  /// The guild where the user has the permissions.
  pub to: u64,
  /// The user's roles on the Guild.
  pub roles: Vec<u64>,
  /// The user's permission on the Guild.
  pub global_permissions: u64,
}

impl PermissionsOn {
  /// Takes a [`Member`] and creates a database record template.
  pub async fn from_member(m: &Member, ctx: &Context) -> Self {
    Self {
      from: m.user.id.get(),
      to: m.guild_id.get(),
      roles: m.roles(ctx).unwrap().iter().map(|r| r.id.get()).collect(),
      global_permissions: m.permissions(ctx).unwrap().bits(),
    }
  }
}
