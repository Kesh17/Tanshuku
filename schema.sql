create sequence IF NOT EXISTS "id_seq" START 1;

create table if not exists "short_urls" (
    id bigint primary key,
    long_url text not null,
    short_url text not null unique,
    short_code text not null unique
);
