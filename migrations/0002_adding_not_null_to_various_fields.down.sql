-- revert the changes made to add not null to uuid and timestamp fields
alter table persons alter column alt_id drop not null;
alter table persons alter column created_at drop not null;
alter table persons alter column last_update drop not null;
alter table invoices alter column alt_id drop not null;
alter table invoices alter column created_at drop not null;
alter table invoices alter column last_update drop not null;
alter table item alter column alt_id drop not null;
alter table item alter column created_at drop not null;
alter table item alter column last_update drop not null;

