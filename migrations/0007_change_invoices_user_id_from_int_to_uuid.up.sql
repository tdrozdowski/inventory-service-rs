-- Add up migration script here
ALTER TABLE invoices
    DROP COLUMN user_id,
    ADD COLUMN user_id uuid NOT NULL;
