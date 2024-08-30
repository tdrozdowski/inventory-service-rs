-- reverse the changes made in the "up" migration

ALTER TABLE invoices_items
    ADD COLUMN new_invoice_id int;
ALTER TABLE invoices_items
    ADD COLUMN new_item_id int;

ALTER TABLE invoices_items
    DROP COLUMN invoice_id;
ALTER TABLE invoices_items
    DROP COLUMN item_id;

ALTER TABLE invoices_items
    RENAME COLUMN new_invoice_id TO invoice_id;
ALTER TABLE invoices_items
    RENAME COLUMN new_item_id TO item_id;


ALTER TABLE invoices_items
    DROP IF EXISTS CONSTRAINT fk_invoices_items_invoice_id;
ALTER TABLE invoices_items
    DROP IF EXISTS CONSTRAINT fk_invoices_items_item_id;
