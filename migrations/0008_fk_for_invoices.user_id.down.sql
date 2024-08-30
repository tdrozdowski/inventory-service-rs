-- reverses the migration in 0008_fk_for_invoices.user_id.up.sql
ALTER TABLE invoices
    DROP CONSTRAINT fk_invoices_user_id;
