-- Add not null to invoices_items columns
ALTER TABLE invoices_items
    ALTER COLUMN invoice_id SET NOT NULL,
    ALTER COLUMN item_id SET NOT NULL;