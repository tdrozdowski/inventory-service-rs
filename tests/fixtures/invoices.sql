-- generate 23 rows for the inventory.invoices table, with the following columns:
--  - alt_id: uuid
--  - user_id: uuid
--  - total: decimal
--  - paid: boolean
--  - created_by: text
--  - last_changed_by: text

INSERT INTO invoices (alt_id, user_id, total, paid, created_by, last_changed_by)
VALUES ('6f4bdd88-d12e-421a-bac7-92ed2d9035ba', '2b1b425e-dee2-4227-8d94-f470a0ce0cd0', 100.00, false, 'unit_test',
        'unit_test'),
       ('2492b388-e0b9-47ca-97a1-8f5ba75441ba', '2b1b425e-dee2-4227-8d94-f470a0ce0cd0', 200.00, false, 'unit_test',
        'unit_test'),
       (gen_random_uuid(), '2d04eae8-ec44-4a9d-9ab5-c6cccf5c8588', 300.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2d04eae8-ec44-4a9d-9ab5-c6cccf5c8588', 400.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2b1b425e-dee2-4227-8d94-f470a0ce0cd0', 500.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2b1b425e-dee2-4227-8d94-f470a0ce0cd0', 600.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2d04eae8-ec44-4a9d-9ab5-c6cccf5c8588', 700.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2d04eae8-ec44-4a9d-9ab5-c6cccf5c8588', 800.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2b1b425e-dee2-4227-8d94-f470a0ce0cd0', 900.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2b1b425e-dee2-4227-8d94-f470a0ce0cd0', 1000.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2b1b425e-dee2-4227-8d94-f470a0ce0cd0', 1100.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2d04eae8-ec44-4a9d-9ab5-c6cccf5c8588', 1200.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2d04eae8-ec44-4a9d-9ab5-c6cccf5c8588', 1300.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2b1b425e-dee2-4227-8d94-f470a0ce0cd0', 1400.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2d04eae8-ec44-4a9d-9ab5-c6cccf5c8588', 1500.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2d04eae8-ec44-4a9d-9ab5-c6cccf5c8588', 1600.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2b1b425e-dee2-4227-8d94-f470a0ce0cd0', 1700.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2b1b425e-dee2-4227-8d94-f470a0ce0cd0', 1800.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2d04eae8-ec44-4a9d-9ab5-c6cccf5c8588', 1900.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2d04eae8-ec44-4a9d-9ab5-c6cccf5c8588', 2000.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2b1b425e-dee2-4227-8d94-f470a0ce0cd0', 2100.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2b1b425e-dee2-4227-8d94-f470a0ce0cd0', 2200.00, false, 'unit_test', 'unit_test'),
       (gen_random_uuid(), '2b1b425e-dee2-4227-8d94-f470a0ce0cd0', 2300.00, false, 'unit_test', 'unit_test');

-- associate some items from items.sql with the invoices from above

INSERT INTO invoices_items (invoice_id, item_id)
values ('6f4bdd88-d12e-421a-bac7-92ed2d9035ba', '6f4bdd88-d12e-421a-bac7-92ed2d9035aa'),
       ('6f4bdd88-d12e-421a-bac7-92ed2d9035ba', '2492b388-e0b9-47ca-97a1-8f5ba75441ea'),
       ('2492b388-e0b9-47ca-97a1-8f5ba75441ba', '6f4bdd88-d12e-421a-bac7-92ed2d9035aa'),
       ('2492b388-e0b9-47ca-97a1-8f5ba75441ba', '2492b388-e0b9-47ca-97a1-8f5ba75441ea');
