-- reverse the changes made in 0010_add_not_null_to_invoices_items_columns.up.sql
ALTER TABLE invoices_items
    ALTER COLUMN invoice_id DROP NOT NULL,
    ALTER COLUMN item_id DROP NOT NULL;

