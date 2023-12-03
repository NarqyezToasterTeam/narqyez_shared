/// Feature relations.
pub mod features;
/// The relations between discord's structs.
pub mod relations;

use serde::{Deserialize, Serialize};
use serenity::all::{Context, Guild, GuildId, User as DiscordUser, UserId};
use serenity::Error;

/// This struct represents the data stored in our database about the guilds using our bot.
#[derive(Deserialize, Serialize)]
pub struct Server {
  /// The guild's ID.
  pub id: u64,
  /// The newsletter channel if any.
  pub newsletter_channel: Option<u64>,
  /// The general channel if any.
  ///
  /// Is set to [`Guild.system_channel_id`] if any.
  pub general_channel: Option<u64>,
}

impl Server {
  /// Takes a [`Guild`] and creates a database record template.
  pub async fn from_guild(g: &Guild) -> Self {
    Self {
      id: g.id.get(),
      newsletter_channel: None,
      general_channel: g.system_channel_id.map(|c| c.get()),
    }
  }

  /// Takes a [`GuildId`] and creates a database record template.
  ///
  /// It is preferable to use [`Server::from_guild`].
  pub async fn from_guild_id(i: &GuildId, ctx: &Context) -> Result<Self, Error> {
    let g = match i.to_partial_guild(&ctx).await {
      Ok(s) => s,
      Err(s) => return Err(s),
    };
    Ok(Self {
      id: g.id.get(),
      newsletter_channel: None,
      general_channel: g.system_channel_id.map(|c| c.get()),
    })
  }
}

/// Represents the data we keep on our databases about the users of our bot.
#[derive(Deserialize, Serialize)]
pub struct User {
  /// The id of the user.
  pub id: u64,
  /// Whether the user has accepted the terms of use.
  pub accepted_terms: bool,
}

impl User {
  /// Creates a new instance of [`User`] from a [`DiscordUser`] object.
  pub async fn from_user(u: &DiscordUser) -> Self {
    Self {
      id: u.id.get(),
      accepted_terms: false,
    }
  }

  /// Creates a new instance of [`User`] from a [`UserId`] object.
  pub async fn from_user_id(u: &UserId) -> Self {
    Self {
      id: u.get(),
      accepted_terms: false,
    }
  }
}
