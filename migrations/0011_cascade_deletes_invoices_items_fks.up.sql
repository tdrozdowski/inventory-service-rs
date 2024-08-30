-- add on delete cascade to invoices_items foreign keys
ALTER TABLE invoices_items
    DROP CONSTRAINT fk_invoices_items_invoice_id,
    DROP CONSTRAINT fk_invoices_items_item_id;
ALTER TABLE invoices_items
    ADD CONSTRAINT fk_invoices_items_invoice_alt_id FOREIGN KEY (invoice_id) REFERENCES invoices (alt_id) ON DELETE CASCADE,
    ADD CONSTRAINT fk_invoices_items_item_alt_id FOREIGN KEY (item_id) REFERENCES items (alt_id) ON DELETE CASCADE;

