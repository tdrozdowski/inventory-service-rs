#[cfg(test)]
mod tests {
    use inventory_service::inventory::model::{CreateItemRequest, UpdateItemRequest};
    use inventory_service::inventory::repositories::item::{
        ItemRepository, ItemRepositoryImpl, ItemRow,
    };
    use inventory_service::inventory::repositories::RepoError;
    use inventory_service::test_helpers::{
        first_item_uuid, invalid_uuid, FIRST_ITEM_ID, FIRST_ITEM_UUID,
    };
    use sqlx::PgPool;
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
    #[sqlx::test(migrations = "./migrations", fixtures("items"))]
    async fn test_get_all_items(pool: PgPool) {
        init();
        let repository = ItemRepositoryImpl::new(pool).await;
        let result = repository.get_all_items(None, 10).await;
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 10);
        // get the next page
        let result = repository.get_all_items(Some(items[9].id), 10).await;
        assert!(result.is_ok());
        let items_page2 = result.unwrap();
        assert_eq!(items_page2.len(), 10);
        // get the final page
        let result = repository
            .get_all_items(Some(items_page2[9].id), 10)
            .await;
        assert!(result.is_ok());
        let items_page3 = result.unwrap();
        assert_eq!(items_page3.len(), 3);
        // test there are no further pages
        let result = repository
            .get_all_items(Some(items_page3[2].id), 10)
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
        let result = repository.get_item_by_uuid(FIRST_ITEM_UUID).await;
        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.name, "Item 1");
    }
}