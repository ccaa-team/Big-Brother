create table settings (
  guild text not null unique,
  board_threshold int4,
  board_channel text,
  reply_role text,
  primary key (guild)
)
