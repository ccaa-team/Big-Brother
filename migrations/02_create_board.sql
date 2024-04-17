-- Twilight serializes Ids to strings
create table if not exists board (
  message text not null,
  guild_id text not null,
  channel_id text not null,
  message_id text not null unique,
  post_id text unique,
  stars int not null
)
