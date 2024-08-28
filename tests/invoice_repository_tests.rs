#[cfg(test)]
mod tests {
    use bigdecimal::{BigDecimal, FromPrimitive};
    use inventory_service::inventory::model::{
        CreateInvoiceRequest, Pagination, UpdateInvoiceRequest,
    };
    use inventory_service::inventory::repositories::invoice::{
        InvoiceItemRow, InvoiceRepository, InvoiceRepositoryImpl,
    };
    use inventory_service::inventory::repositories::person::{
        PersonRepository, PersonRepositoryImpl,
    };
    use inventory_service::inventory::repositories::RepoError;
    use inventory_service::test_helpers::{
        first_invoice_uuid, first_item_uuid, first_person_uuid, init, FIRST_INVOICE_ID,
    };
    use sqlx::PgPool;
    use uuid::Uuid;

    #[sqlx::test(fixtures("people", "items", "invoices"))]
    async fn test_get_all_invoices(pool: PgPool) {
        init();
        let repository = InvoiceRepositoryImpl::new(pool).await;
        let result = repository
            .get_all_invoices(Some(Pagination::default()))
            .await;
        assert!(result.is_ok());
        let invoices = result.unwrap();
        assert_eq!(invoices.len(), 10);
        // get the next page
        let mut page = Pagination {
            last_id: Some(invoices[9].id),
            page_size: 10,
        };
        let result = repository.get_all_invoices(Some(page)).await;
        assert!(result.is_ok());
        let invoices_page2 = result.unwrap();
        assert_eq!(invoices_page2.len(), 10);
        // get the final page
        page.last_id = Some(invoices_page2[9].id);
        let result = repository.get_all_invoices(Some(page)).await;
        assert!(result.is_ok());
        let invoices_page3 = result.unwrap();
        assert_eq!(invoices_page3.len(), 3);
        // test there are no further pages
        page.last_id = Some(invoices_page3[2].id);
        let result = repository.get_all_invoices(Some(page)).await;
        assert!(result.is_ok());
        let invoices_page4 = result.unwrap();
        assert_eq!(invoices_page4.len(), 0);
    }

    #[sqlx::test(fixtures("people", "items", "invoices"))]
    async fn test_create_invoice(pool: PgPool) {
        init();
        let repository = InvoiceRepositoryImpl::new(pool).await;
        let invoice_request = CreateInvoiceRequest {
            user_id: first_person_uuid(),
            total: 100.0,
            paid: false,
            created_by: "unit_test".to_string(),
            items: vec![],
        };
        let result = repository.create(invoice_request).await;
        assert!(result.is_ok());
        let invoice = result.unwrap();
        assert_eq!(invoice.total, BigDecimal::from_f64(100.0).unwrap());
    }

    #[sqlx::test(fixtures("people", "items", "invoices"))]
    async fn test_get_invoice_by_id(pool: PgPool) {
        init();
        let repository = InvoiceRepositoryImpl::new(pool).await;
        let result = repository.get_by_id(FIRST_INVOICE_ID).await;
        assert!(result.is_ok());
        let invoice = result.unwrap();
        assert_eq!(invoice.total, BigDecimal::from_f64(100.0).unwrap());
    }

    #[sqlx::test(fixtures("people", "items", "invoices"))]
    async fn test_get_invoice_by_uuid(pool: PgPool) {
        init();
        let repository = InvoiceRepositoryImpl::new(pool).await;
        let result = repository.get_by_uuid(first_invoice_uuid()).await;
        assert!(result.is_ok());
        let invoice = result.unwrap();
        assert_eq!(invoice.total, BigDecimal::from_f64(100.0).unwrap());
    }

    #[sqlx::test(fixtures("people", "items", "invoices"))]
    async fn test_update_invoice(pool: PgPool) {
        init();
        let repository = InvoiceRepositoryImpl::new(pool).await;
        let update_invoice_request = UpdateInvoiceRequest {
            id: first_invoice_uuid(),
            total: 200000.0,
            paid: true,
            changed_by: "unit_test".to_string(),
        };
        let result = repository.update(update_invoice_request).await;
        assert!(result.is_ok());
        let updated_invoice = result.unwrap();
        assert_eq!(
            updated_invoice.total,
            BigDecimal::from_f64(200000.0).unwrap()
        );
    }

    #[sqlx::test(fixtures("people", "items", "invoices"))]
    async fn test_find_invoices_by_user_id(pool: PgPool) {
        init();
        let repository = InvoiceRepositoryImpl::new(pool).await;
        let result = repository.find_by_user_id(first_person_uuid()).await;
        assert!(result.is_ok());
        let invoices = result.unwrap();
        assert_eq!(invoices.len(), 13);
    }

    #[sqlx::test(fixtures("people", "items", "invoices"))]
    async fn test_find_invoices_by_user_id_empty(pool: PgPool) {
        init();
        let cloned_pool = pool.clone();
        let person_repository = PersonRepositoryImpl::new(pool).await;
        let result = person_repository.get_all_persons(None, 5).await;
        assert!(result.is_ok());
        let persons = result.unwrap();
        assert_eq!(persons.len(), 5);
        let person_id = persons[2].alt_id;
        let repository = InvoiceRepositoryImpl::new(cloned_pool).await;
        let result = repository.find_by_user_id(person_id).await;
        assert!(result.is_ok());
        let invoices = result.unwrap();
        assert_eq!(invoices.len(), 0);
    }

    #[sqlx::test(fixtures("people", "items", "invoices"))]
    async fn test_delete_invoice(pool: PgPool) {
        init();
        let repository = InvoiceRepositoryImpl::new(pool).await;
        let result = repository.delete(first_invoice_uuid()).await;
        assert!(result.is_ok());
        let delete_resutls = result.unwrap();
        assert_eq!(delete_resutls.id, first_invoice_uuid().to_string());
        assert_eq!(delete_resutls.deleted, true);
    }

    #[sqlx::test(fixtures("people", "items", "invoices"))]
    async fn test_delete_invoice_not_found(pool: PgPool) {
        init();
        let repository = InvoiceRepositoryImpl::new(pool).await;
        let result = repository.delete(first_invoice_uuid()).await;
        assert!(result.is_ok());
        let delete_results = result.unwrap();
        assert_eq!(delete_results.id, first_invoice_uuid().to_string());
        assert_eq!(delete_results.deleted, true);
        let result = repository.delete(first_invoice_uuid()).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        match error {
            RepoError::NotFound(_) => assert!(true),
            _ => assert!(false, "Expected NotFound error"),
        }
    }

    #[sqlx::test(fixtures("people", "items", "invoices"))]
    async fn test_add_item_to_invoice(pool: PgPool) {
        init();
        let repository = InvoiceRepositoryImpl::new(pool).await;
        let create_invoice_request = CreateInvoiceRequest {
            user_id: first_person_uuid(),
            total: 100.0,
            paid: false,
            created_by: "unit_test".to_string(),
            items: vec![],
        };
        let result = repository.create(create_invoice_request).await;
        assert!(result.is_ok());
        let invoice = result.unwrap();
        assert_eq!(invoice.total, BigDecimal::from_f64(100.0).unwrap());
        let cloned_invoice_id = invoice.alt_id.clone();
        let invoice_item = InvoiceItemRow {
            invoice_id: invoice.alt_id,
            item_id: first_item_uuid(),
        };
        let result = repository.add_item(invoice_item).await;
        assert!(result.is_ok());
        let invoice_item = result.unwrap();
        assert_eq!(invoice_item.invoice_id, cloned_invoice_id);
        assert_eq!(invoice_item.item_id, first_item_uuid());
    }

    #[sqlx::test(fixtures("people", "items", "invoices"))]
    async fn test_add_item_to_invoice_invalid_uuid(pool: PgPool) {
        init();
        let repository = InvoiceRepositoryImpl::new(pool).await;
        let invoice_item = InvoiceItemRow {
            invoice_id: Uuid::new_v4(),
            item_id: first_item_uuid(),
        };
        let result = repository.add_item(invoice_item).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        match error {
            RepoError::Other(_) => assert!(true),
            e => assert!(false, "Expected Other error, received: {:?}", e),
        }
    }

    #[sqlx::test(fixtures("people", "items", "invoices"))]
    async fn test_add_item_to_invoice_invalid_item_uuid(pool: PgPool) {
        init();
        let repository = InvoiceRepositoryImpl::new(pool).await;
        let invoice_item = InvoiceItemRow {
            invoice_id: first_invoice_uuid(),
            item_id: Uuid::new_v4(),
        };
        let result = repository.add_item(invoice_item).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        match error {
            RepoError::Other(_) => assert!(true),
            e => assert!(false, "Expected Other error, received: {:?}", e),
        }
    }

    #[sqlx::test(fixtures("people", "items", "invoices"))]
    async fn test_remove_item_from_invoice(pool: PgPool) {
        init();
        let repository = InvoiceRepositoryImpl::new(pool).await;
        let create_invoice_request = CreateInvoiceRequest {
            user_id: first_person_uuid(),
            total: 100.0,
            paid: false,
            created_by: "unit_test".to_string(),
            items: vec![],
        };
        let result = repository.create(create_invoice_request).await;
        assert!(result.is_ok());
        let invoice = result.unwrap();
        assert_eq!(invoice.total, BigDecimal::from_f64(100.0).unwrap());
        let cloned_invoice_id = invoice.alt_id.clone();
        let invoice_item = InvoiceItemRow {
            invoice_id: invoice.alt_id,
            item_id: first_item_uuid(),
        };
        let result = repository.add_item(invoice_item).await;
        assert!(result.is_ok());
        let invoice_item = result.unwrap();
        assert_eq!(invoice_item.invoice_id, cloned_invoice_id);
        assert_eq!(invoice_item.item_id, first_item_uuid());
        let result = repository.remove_item(invoice_item).await;
        assert!(result.is_ok());
        let delete_results = result.unwrap();
        assert_eq!(delete_results.deleted, true);
    }
}
