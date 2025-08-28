-- Add up migration script here
create table if not exists literature_notes (
    id integer primary key autoincrement,
    content text,
    create_time DATETIME not null,
    update_time DATETIME not null
);

-- 创建literature和literature之间的关联表
create table if not exists literature_relations (
    id integer primary key autoincrement,
    literature_id integer,
    relation_id integer,
    relation_type text,
    create_time DATETIME not null,
    update_time DATETIME not null
);

DROP TABLE IF EXISTS tags;

-- 创建tag表，用于存储标签
create table if not exists tags (
    id integer primary key autoincrement,
    full_name text,
    p_id integer,
    create_time DATETIME not null,
    update_time DATETIME not null
);