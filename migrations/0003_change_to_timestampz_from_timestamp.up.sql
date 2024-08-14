-- changing timestamp to timestampz
alter table persons alter column created_at type timestamp with time zone using created_at at time zone 'utc';
alter table persons alter column last_update type timestamp with time zone using last_update at time zone 'utc';
alter table invoices alter column created_at type timestamp with time zone using created_at at time zone 'utc';
alter table invoices alter column last_update type timestamp with time zone using last_update at time zone 'utc';
alter table item alter column created_at type timestamp with time zone using created_at at time zone 'utc';
alter table item alter column last_update type timestamp with time zone using last_update at time zone 'utc';

