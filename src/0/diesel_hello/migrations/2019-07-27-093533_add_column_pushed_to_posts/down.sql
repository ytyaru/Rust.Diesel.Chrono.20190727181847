-- BEGIN TRANSACTION;

-- 既存のテーブルをリネーム
ALTER TABLE posts RENAME TO tmp_posts;
-- 新しいテーブルを作成（元々のテーブル名と同じ名前で）
create table posts (
    id integer primary key,
    title text not null,
    body text not null,
    is_published text not null default 'f'
);
-- レコードを全て移す
INSERT INTO posts(id, title, body, is_published) SELECT id, title, body, is_published FROM tmp_posts;
-- 元のテーブルを削除
DROP TABLE tmp_posts;

-- COMMIT;
