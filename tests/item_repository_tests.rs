#![feature(assert_matches)]

#[cfg(test)]
mod tests {
    use bigdecimal::{BigDecimal, FromPrimitive};
    use inventory_service::inventory::model::{CreateItemRequest, Pagination, UpdateItemRequest};
    use inventory_service::inventory::repositories::item::{
        ItemRepository, ItemRepositoryImpl,
    };
    use inventory_service::inventory::repositories::RepoError;
    use inventory_service::test_helpers::{first_item_uuid, invalid_uuid, FIRST_ITEM_ID, FIRST_ITEM_UUID};
    use sqlx::PgPool;
    use std::assert_matches::assert_matches;
    use std::sync::Once;
    use tracing::Level;

    static TRACING: Once = Once::new();
    pub fn init() {
        TRACING.call_once(|| {
            tracing_subscriber::fmt()
                .with_max_level(Level::DEBUG)
                .init();
        });
    }

    #[sqlx::test(fixtures("items"))]
    async fn test_get_all_items(pool: PgPool) {
        init();
        let repository = ItemRepositoryImpl::new(pool).await;
        let result = repository.get_all_items(Some(Pagination::default())).await;
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 10);
        // get the next page
        let mut page = Pagination {
            last_id: Some(items[9].id),
            page_size: 10,
        };
        let result = repository.get_all_items(Some(page)).await;
        assert!(result.is_ok());
        let items_page2 = result.unwrap();
        assert_eq!(items_page2.len(), 10);
        // get the final page
        page.last_id = Some(items_page2[9].id);
        let result = repository
            .get_all_items(Some(page))
            .await;
        assert!(result.is_ok());
        let items_page3 = result.unwrap();
        assert_eq!(items_page3.len(), 3);
        // test there are no further pages
        page.last_id = Some(items_page3[2].id);
        let result = repository
            .get_all_items(Some(page))
            .await;
        assert!(result.is_ok());
        let items_page4 = result.unwrap();
        assert_eq!(items_page4.len(), 0);
    }

    // test all functions on ItemRepositoryImpl
    #[sqlx::test(fixtures("items"))]
    async fn test_get_item_by_id(pool: PgPool) {
        init();
        let repository = ItemRepositoryImpl::new(pool).await;
        let result = repository.get_item_by_id(FIRST_ITEM_ID).await;
        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.name, "Item 1");
    }
    #[sqlx::test(fixtures("items"))]
    async fn test_get_item_by_uuid(pool: PgPool) {
        init();
        let repository = ItemRepositoryImpl::new(pool).await;
        let result = repository.get_item_by_uuid(first_item_uuid()).await;
        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.name, "Item 1");
    }

    #[sqlx::test(fixtures("items"))]
    async fn test_create_item(pool: PgPool) {
        init();
        let repository = ItemRepositoryImpl::new(pool).await;
        let item_request = CreateItemRequest {
            name: "Test Item".to_string(),
            description: "Test Item Description".to_string(),
            unit_price: 100.0,
            created_by: "testuser".to_string(),
        };
        let result = repository.create_item(&item_request).await;
        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.description, "Test Item Description");
        assert_eq!(item.unit_price, BigDecimal::from_f64(100.0).unwrap());
    }

    #[sqlx::test(fixtures("items"))]
    async fn test_update_item(pool: PgPool) {
        init();
        let repository = ItemRepositoryImpl::new(pool).await;
        let item_request = UpdateItemRequest {
            id: FIRST_ITEM_UUID.to_string(),
            name: "Updated Item".to_string(),
            description: "Updated Item Description".to_string(),
            unit_price: 200.0,
            changed_by: "testuser".to_string(),
        };
        let result = repository.update_item(&item_request).await;
        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.name, "Updated Item");
        assert_eq!(item.description, "Updated Item Description");
        assert_eq!(item.unit_price, BigDecimal::from_f64(200.0).unwrap());
    }

    #[sqlx::test(fixtures("items"))]
    async fn test_delete_item(pool: PgPool) {
        init();
        let repository = ItemRepositoryImpl::new(pool).await;
        let result = repository.delete_item(first_item_uuid()).await;
        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.name, "Item 1");
    }

    #[sqlx::test(fixtures("items"))]
    async fn test_get_item_by_id_not_found(pool: PgPool) {
        init();
        let repository = ItemRepositoryImpl::new(pool).await;
        let result = repository.get_item_by_id(999).await;
        assert_eq!(result.is_err(), true);
        match result.unwrap_err() {
            RepoError::NotFound(_) => (),
            _ => assert!(false, "Expected NotFound error"),
        }
    }

    #[sqlx::test(fixtures("items"))]
    async fn test_get_item_by_uuid_not_found(pool: PgPool) {
        init();
        let repository = ItemRepositoryImpl::new(pool).await;
        let result = repository.get_item_by_uuid(invalid_uuid()).await;
        assert_eq!(result.is_err(), true);
        match result.unwrap_err() {
            RepoError::NotFound(_) => (),
            _ => assert!(false, "Expected NotFound error"),
        }
    }

    #[sqlx::test(fixtures("items"))]
    async fn test_update_item_not_found(pool: PgPool) {
        init();
        let repository = ItemRepositoryImpl::new(pool).await;
        let item_request = UpdateItemRequest {
            id: "00000000-0000-0000-0000-000000000000".to_string(),
            name: "Updated Item".to_string(),
            description: "Updated Item Description".to_string(),
            unit_price: 200.0,
            changed_by: "testuser".to_string(),
        };
        let result = repository.update_item(&item_request).await;
        assert_eq!(result.is_err(), true);
        match result.unwrap_err() {
            RepoError::NotFound(_) => (),
            _ => assert!(false, "Expected NotFound error"),
        }
    }

    #[sqlx::test(fixtures("items"))]
    async fn test_delete_item_not_found(pool: PgPool) {
        init();
        let repository = ItemRepositoryImpl::new(pool).await;
        let result = repository.delete_item(invalid_uuid()).await;
        assert_eq!(result.is_err(), true);
        match result.unwrap_err() {
            RepoError::NotFound(_) => (),
            _ => assert!(false, "Expected NotFound error"),
        }
    }
}