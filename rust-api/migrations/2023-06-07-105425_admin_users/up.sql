-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table if not exists admin_users
(
    id         uuid         default uuid_generate_v4()             not null primary key,
    name       varchar(255)                                        not null,
    email      varchar(255)                                        not null unique,
    password   varchar(255)                                        not null,
    created_at timestamp    default now()                          not null,
    updated_at timestamp    default now()                          not null,
    created_by varchar(255) default 'anonymous'::character varying not null,
    updated_by varchar(255) default 'anonymous'::character varying not null
);
