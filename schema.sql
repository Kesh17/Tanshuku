create table if not exists "short_urls" (
    long_url text not null,
    short_url text not null unique,
    short_code text not null unique
);
