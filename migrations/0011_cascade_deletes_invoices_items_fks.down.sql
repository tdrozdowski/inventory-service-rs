-- reverses the changes in 0011_cascade_deletes_invoices_items_fks.up.sql
ALTER TABLE invoices_items
    DROP CONSTRAINT fk_invoices_items_invoice_alt_id,
    DROP CONSTRAINT fk_invoices_items_item_alt_id;
ALTER TABLE invoices_items
    ADD CONSTRAINT fk_invoices_items_invoice_id FOREIGN KEY (invoice_id) REFERENCES invoices (id),
    ADD CONSTRAINT fk_invoices_items_item_id FOREIGN KEY (item_id) REFERENCES items (id);
