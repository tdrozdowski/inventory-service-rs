-- set invoices.paid column to not null
ALTER TABLE invoices
    ALTER COLUMN paid SET NOT NULL;
