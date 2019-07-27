-- 投稿
create table posts (
    id integer primary key,
    title text not null,
    body text not null,
    is_published text not null default 'f'
);

