-- change invoices_items.invoice_id and invoices_items.item_id type to UUID and add a foreign key constraint
ALTER TABLE invoices_items
    ADD COLUMN new_invoice_id uuid;
ALTER TABLE invoices_items
    ADD COLUMN new_item_id uuid;

ALTER TABLE invoices_items
    DROP COLUMN invoice_id;
ALTER TABLE invoices_items
    DROP COLUMN item_id;

ALTER TABLE invoices_items
    RENAME COLUMN new_invoice_id TO invoice_id;
ALTER TABLE invoices_items
    RENAME COLUMN new_item_id TO item_id;

ALTER TABLE invoices_items
    ADD CONSTRAINT fk_invoices_items_invoice_id FOREIGN KEY (invoice_id) REFERENCES invoices (alt_id),
    ADD CONSTRAINT fk_invoices_items_item_id FOREIGN KEY (item_id) REFERENCES items (alt_id);

