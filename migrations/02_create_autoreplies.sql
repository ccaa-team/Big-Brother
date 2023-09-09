create table if not exists replies (
  trigger varchar(128) not null unique,
  reply varchar(256) not null
);
