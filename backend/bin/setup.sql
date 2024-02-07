drop table if exists users;
create table users (
    id UUID primary key,
    name text not null,
    email text not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
    );

-- insert into users (id, name, email) 
-- values ('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'abc', 'abc@gmail.com');


-- drop table if exists tweet;
-- create table tweet (
--     user_id UUID key,
--     name text not null,
--     email text not null,
--     created_at timestamp not null default now(),
--     updated_at timestamp not null default now()
--     );
