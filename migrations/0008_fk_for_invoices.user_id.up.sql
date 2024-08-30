-- Adds FK from invoices.user_id to persons.alt_id
ALTER TABLE invoices
    ADD CONSTRAINT fk_invoices_user_id
        FOREIGN KEY (user_id)
            REFERENCES persons (alt_id)
            ON DELETE CASCADE;
