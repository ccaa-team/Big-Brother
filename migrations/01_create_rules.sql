create table if not exists rules (
  trigger varchar(512) not null unique,
  reply varchar(256) not null,
  guild text not null
)
