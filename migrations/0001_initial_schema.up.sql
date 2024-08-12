create extension if not exists "pgcrypto";

create table persons (
    id serial primary key,
    alt_id uuid default gen_random_uuid() unique,
    name text not null,
    email text unique not null,
    created_by text not null,
    created_at timestamp default now(),
    last_update timestamp default now()
);

create table if not exists invoices (
    id serial primary key,
    alt_id uuid default gen_random_uuid() unique,
    user_id int not null,
    total decimal not null,
    paid boolean default false,
    created_by text not null,
    created_at timestamp default now(),
    last_update timestamp default now()
);

create table item (
  id serial primary key,
  alt_id uuid default gen_random_uuid() unique,
  name varchar(255) not null,
  description text not null,
  unit_price decimal not null,
  created_by text not null,
  created_at timestamp default now(),
  last_update timestamp default now()
);

create table if not exists invoices_items (
    invoice_id int not null,
    item_id int not null
);
