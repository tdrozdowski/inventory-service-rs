-- add not null to uuid and timestamp fields
alter table persons alter column alt_id set not null;
alter table persons alter column created_at set not null;
alter table persons alter column last_update set not null;
alter table invoices alter column alt_id set not null;
alter table invoices alter column created_at set not null;
alter table invoices alter column last_update set not null;
alter table item alter column alt_id set not null;
alter table item alter column created_at set not null;
alter table item alter column last_update set not null;

