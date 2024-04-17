create table settings (
  guild text not null unique,
  board_threshold smallint not null,
  board_channel text,
  primary key (guild)
)
