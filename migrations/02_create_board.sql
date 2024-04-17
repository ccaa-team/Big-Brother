-- Twilight serializes Ids to strings
create table board (
  message text not null,
  guild_id text not null,
  message_id text not null unique,
  post_id text unique,
  stars int not null
)
