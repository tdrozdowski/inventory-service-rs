-- reverses the migration in 0009_add_not_null_to_invoice_paid.up.sql
ALTER TABLE invoices
    ALTER COLUMN paid DROP NOT NULL;