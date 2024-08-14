-- revert changes made to change timestamp to timestampz
alter table persons alter column created_at type timestamp using created_at;
alter table persons alter column last_update type timestamp using last_update;
alter table invoices alter column created_at type timestamp using created_at;
alter table invoices alter column last_update type timestamp using last_update;
alter table item alter column created_at type timestamp using created_at;
alter table item alter column last_update type timestamp using last_update;

