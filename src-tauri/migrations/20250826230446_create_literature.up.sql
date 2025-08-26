-- Add up migration script here
create table if not exists literature_notes (
    id integer primary key autoincrement,
    content text,
    create_time timestamp default current_timestamp,
    update_time timestamp default current_timestamp
);

-- 创建literature和literature之间的关联表
create table if not exists literature_relations (
    id integer primary key autoincrement,
    literature_id integer,
    relation_id integer,
    relation_type text,
    create_time timestamp default current_timestamp,
    update_time timestamp default current_timestamp
);

-- 创建tag表，用于存储标签
create table if not exists tags (
    id integer primary key autoincrement,
    name text,
    create_time timestamp default current_timestamp,
    update_time timestamp default current_timestamp
);