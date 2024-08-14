-- add a new non nullable text column to the audit named last_changed_by to all tables
alter table persons add column last_changed_by text not null default 'system';
alter table invoices add column last_changed_by text not null default 'system';
alter table item add column last_changed_by text not null default 'system';

