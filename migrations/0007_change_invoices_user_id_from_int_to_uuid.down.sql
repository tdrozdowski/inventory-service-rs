-- reverse the changes made in the "up" migration
ALTER TABLE invoices
    DROP COLUMN user_id,
    ADD COLUMN user_id int NOT NULL;
