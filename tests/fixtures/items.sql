-- insert 25 items for the items table, including alt_id
INSERT INTO items (alt_id, name, description, unit_price, created_by, last_changed_by)
VALUES ('6f4bdd88-d12e-421a-bac7-92ed2d9035aa', 'Item 1', 'Item 1 description', 10.00, 'unit_test', 'unit_test'),
       ('2492b388-e0b9-47ca-97a1-8f5ba75441ea', 'Item 2', 'Item 2 description', 20.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 3', 'Item 3 description', 30.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 4', 'Item 4 description', 40.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 5', 'Item 5 description', 50.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 6', 'Item 6 description', 60.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 7', 'Item 7 description', 70.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 8', 'Item 8 description', 80.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 9', 'Item 9 description', 90.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 10', 'Item 10 description', 100.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 11', 'Item 11 description', 110.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 12', 'Item 12 description', 120.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 13', 'Item 13 description', 130.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 14', 'Item 14 description', 140.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 15', 'Item 15 description', 150.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 16', 'Item 16 description', 160.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 17', 'Item 17 description', 170.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 18', 'Item 18 description', 180.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 19', 'Item 19 description', 190.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 20', 'Item 20 description', 200.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 21', 'Item 21 description', 210.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 22', 'Item 22 description', 220.00, 'unit_test', 'unit_test'),
       (gen_random_uuid(), 'Item 23', 'Item 23 description', 230.00, 'unit_test', 'unit_test');

