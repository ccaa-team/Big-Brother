create table if not exists moyai (
  message_id varchar(32) not null unique,
  post_id varchar(32) unique,
  message_content varchar(4000) not null,
  moyai_count INT(32) not null,
  author varchar(88) not null
);
