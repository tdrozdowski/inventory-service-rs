create table persons (
    id serial primary key,
    alt_id uuid unique gen_random_uuid(),
    name text not null,
    email text unique not null,
    created_at timestamp now(),
    last_update timestamp now()
);

create table if not exists invoices (
    id serial primary key,
    alt_id uuid unique gen_random_uuid()
    user_id int not null,
    total decimal not null,
    paid boolean default false,
    created_at timestamp not null
);

create table item (
  id serial primary key,
  alt_id uuid unique gen_random_uuid()
  name varchar(255) not null,
  description text not null,
  unit_price decimal not null,
  created_at timestamp not null
);

create table if not exists invoices_items (
    invoice_id int not null,
    item_id int not null
);
