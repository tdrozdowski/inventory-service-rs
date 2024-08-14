-- revert the changes made to add last_changed_by to all tables
alter table persons drop column last_changed_by;
alter table invoices drop column last_changed_by;
alter table item drop column last_changed_by;
