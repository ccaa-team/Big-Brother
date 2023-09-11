create table if not exists moyai (
  message_id text not null unique,
  post_id text unique,
  message_content text not null,
  moyai_count bigint not null,
  author text not null
);
