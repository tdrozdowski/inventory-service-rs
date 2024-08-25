use crate::inventory::model::{AuditInfo, CreateItemRequest, DeleteResults, Item, Pagination, UpdateItemRequest};
use crate::inventory::repositories::item::{ItemRepository, ItemRow};
use crate::inventory::services::ServiceError;
use crate::test_helpers::string_to_uuid;
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use garde::Validate;
use std::fmt::Debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[async_trait]
#[mockall::automock]
pub trait ItemService: Sync + Send + Debug + 'static {
    async fn get_all_items(&self, maybe_pagination: Option<Pagination>) -> Result<Vec<Item>, ServiceError>;
    async fn get_item_by_id(&self, id: Uuid) -> Result<Item, ServiceError>;
    async fn create_item(&self, item: CreateItemRequest) -> Result<Item, ServiceError>;
    async fn update_item(&self, item: UpdateItemRequest) -> Result<Item, ServiceError>;
    async fn delete_item(&self, id: Uuid) -> Result<DeleteResults, ServiceError>;
}

#[derive(Debug)]
pub struct ItemServiceImpl {
    pub item_repository: Arc<dyn ItemRepository + Send + Sync>,
}

impl ItemServiceImpl {
    pub fn new(item_repository: Arc<dyn ItemRepository + Send + Sync>) -> ItemServiceImpl {
        ItemServiceImpl { item_repository }
    }
}

#[async_trait]
impl ItemService for ItemServiceImpl {
    #[instrument]
    async fn get_all_items(&self, maybe_pagination: Option<Pagination>) -> Result<Vec<Item>, ServiceError> {
        self.item_repository.get_all_items(maybe_pagination).await
            .map(|items| items.into_iter().map(Item::from).collect())
            .map_err(ServiceError::from)
    }

    #[instrument]
    async fn get_item_by_id(&self, id: Uuid) -> Result<Item, ServiceError> {
        self.item_repository.get_item_by_uuid(id).await
            .map(Item::from)
            .map_err(ServiceError::from)
    }

    #[instrument]
    async fn create_item(&self, item: CreateItemRequest) -> Result<Item, ServiceError> {
        if let None = BigDecimal::from_f64(item.unit_price) {
            return Err(ServiceError::InvalidPrice(format!("Invalid unit price: {}", item.unit_price)));
        }
        if let Err(e) = item.validate() {
            return Err(ServiceError::InputValidationError(format!("Invalid input: {}", e)));
        }
        self.item_repository.create_item(&item).await
            .map(Item::from)
            .map_err(ServiceError::from)
    }

    #[instrument]
    async fn update_item(&self, item: UpdateItemRequest) -> Result<Item, ServiceError> {
        if let None = BigDecimal::from_f64(item.unit_price) {
            return Err(ServiceError::InvalidPrice(format!("Invalid unit price: {}", item.unit_price)));
        }
        if let Err(e) = Uuid::parse_str(&item.id) {
            return Err(ServiceError::InvalidUuid(format!("Invalid id: {} - details: {}", item.id, e.clone())));
        }
        if let Err(e) = item.validate() {
            return Err(ServiceError::InputValidationError(format!("Invalid input: {}", e)));
        }
        self.item_repository.update_item(&item).await
            .map(Item::from)
            .map_err(ServiceError::from)
    }

    #[instrument]
    async fn delete_item(&self, id: Uuid) -> Result<DeleteResults, ServiceError> {
        self.item_repository.delete_item(id).await
            .map(|row| DeleteResults { id: String::from(row.alt_id), deleted: true })
            .map_err(ServiceError::from)
    }
}


impl From<ItemRow> for Item {
    fn from(item_row: ItemRow) -> Self {
        Item {
            seq: item_row.id,
            id: String::from(item_row.alt_id),
            name: item_row.name,
            description: item_row.description,
            unit_price: item_row.unit_price.to_f64().unwrap(),
            audit_info: AuditInfo {
                created_by: item_row.created_by,
                created_at: item_row.created_at,
                changed_by: item_row.last_changed_by,
                updated_at: item_row.last_update,
            },
        }
    }
}

impl From<Item> for ItemRow {
    fn from(item: Item) -> Self {
        ItemRow {
            id: item.seq,
            alt_id: string_to_uuid(&item.id),
            name: item.name,
            description: item.description,
            unit_price: BigDecimal::from_f64(item.unit_price).unwrap(),
            created_by: item.audit_info.created_by,
            created_at: item.audit_info.created_at,
            last_changed_by: item.audit_info.changed_by,
            last_update: item.audit_info.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::inventory::model::{AuditInfo, CreateItemRequest, Item, UpdateItemRequest};
    use crate::inventory::repositories::item::{ItemRow, MockItemRepository};
    use crate::inventory::services::item::{ItemService, ItemServiceImpl};
    use crate::test_helpers::string_to_uuid;
    use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
    use chrono::Utc;
    use mockall::predicate::eq;
    use std::assert_matches::assert_matches;
    use std::sync::{Arc, Once};
    use tracing::Level;
    use uuid::Uuid;
    use crate::inventory::services::ServiceError::InputValidationError;

    static TRACING: Once = Once::new();
    pub fn init() {
        TRACING.call_once(|| {
            tracing_subscriber::fmt()
                .with_max_level(Level::DEBUG)
                .init();
        });
    }

    fn create_item(uuid: Uuid, seq: i32) -> Item {
        Item {
            seq,
            id: uuid.to_string(),
            name: "item".to_string(),
            description: "item description".to_string(),
            unit_price: 10.0,
            audit_info: AuditInfo {
                created_by: "unit_test".to_string(),
                created_at: Utc::now(),
                changed_by: "unit_test".to_string(),
                updated_at: Utc::now(),
            },
        }
    }
    #[tokio::test]
    async fn test_create_item() {
        init();
        let mut mock = MockItemRepository::new();
        let item = CreateItemRequest {
            name: "item".to_string(),
            description: "item description".to_string(),
            unit_price: 10.0,
            created_by: "user".to_string(),
        };
        let item_clone = item.clone();
        let item_row = ItemRow {
            id: 1,
            alt_id: string_to_uuid("00000000-0000-0000-0000-000000000001"),
            name: item.name.clone(),
            description: item.description.clone(),
            unit_price: BigDecimal::from_f64(item.unit_price).unwrap(),
            created_by: item.created_by.clone(),
            created_at: Utc::now(),
            last_changed_by: item.created_by.clone(),
            last_update: Utc::now(),
        };
        mock.expect_create_item()
            .with(eq(item))
            .times(1)
            .returning(move |_| {
                let cloned_row = item_row.clone();
                Box::pin(async move { Ok(cloned_row) })
            });
        let service = ItemServiceImpl::new(Arc::new(mock));
        let result = service.create_item(item_clone).await;
        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.name, "item");
        assert_eq!(item.description, "item description");
        assert_eq!(item.unit_price, 10.0);
        assert_eq!(item.audit_info.created_by, "user");
        assert_eq!(item.audit_info.changed_by, "user");
    }

    #[tokio::test]
    async fn test_get_item_by_id() {
        init();
        let mut mock = MockItemRepository::new();
        let uuid = Uuid::new_v4();
        let seq = 1;
        let expected_results = create_item(uuid, seq);
        let item_row = ItemRow::from(expected_results.clone());
        mock.expect_get_item_by_uuid()
            .with(eq(uuid))
            .times(1)
            .returning(move |_| {
                let cloned_row = item_row.clone();
                Box::pin(async move { Ok(cloned_row) })
            });
        let service = ItemServiceImpl::new(Arc::new(mock));
        let result = service.get_item_by_id(uuid).await;
        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.id, expected_results.id);
        assert_eq!(item.name, expected_results.name);
        assert_eq!(item.description, expected_results.description);
        assert_eq!(item.unit_price, expected_results.unit_price.to_f64().unwrap());
        assert_eq!(item.audit_info.created_by, expected_results.audit_info.created_by);
        assert_eq!(item.audit_info.changed_by, expected_results.audit_info.changed_by);
    }

    #[tokio::test]
    async fn test_update_item() {
        init();
        let mut mock = MockItemRepository::new();
        let item_request = UpdateItemRequest {
            id: "00000000-0000-0000-0000-000000000001".to_string(),
            name: "item".to_string(),
            description: "item description".to_string(),
            unit_price: 10.0,
            changed_by: "unit_test".to_string(),
        };
        let item_req_clone = item_request.clone();
        let item_row = ItemRow {
            id: 1,
            alt_id: string_to_uuid("00000000-0000-0000-0000-000000000001"),
            name: item_request.name.clone(),
            description: item_request.description.clone(),
            unit_price: BigDecimal::from_f64(item_request.unit_price).unwrap(),
            created_by: "unit_test".to_string(),
            created_at: Utc::now(),
            last_changed_by: "unit_test".to_string(),
            last_update: Utc::now(),
        };
        mock.expect_update_item()
            .returning(move |_| {
                let cloned_row = item_row.clone();
                Box::pin(async move { Ok(cloned_row) })
            });
        let service = ItemServiceImpl::new(Arc::new(mock));
        let result = service.update_item(item_req_clone).await;
        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.name, item_request.name);
        assert_eq!(item.description, item_request.description);
        assert_eq!(item.unit_price, item_request.unit_price);
        assert_eq!(item.audit_info.created_by, item_request.changed_by);
        assert_eq!(item.audit_info.changed_by, item_request.changed_by);
    }

    #[tokio::test]
    async fn test_delete_item() {
        init();
        let mut mock = MockItemRepository::new();
        let item_row = ItemRow {
            id: 1,
            alt_id: string_to_uuid("00000000-0000-0000-0000-000000000001"),
            name: "item".to_string(),
            description: "item description".to_string(),
            unit_price: BigDecimal::from_f64(10.0).unwrap(),
            created_by: "unit_test".to_string(),
            created_at: Utc::now(),
            last_changed_by: "unit_test".to_string(),
            last_update: Utc::now(),
        };
        mock.expect_delete_item()
            .with(eq(string_to_uuid("00000000-0000-0000-0000-000000000001")))
            .times(1)
            .returning(move |_| {
                let cloned_row = item_row.clone();
                Box::pin(async move { Ok(cloned_row) })
            });
        let service = ItemServiceImpl::new(Arc::new(mock));
        let result = service.delete_item(string_to_uuid("00000000-0000-0000-0000-000000000001")).await;
        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.id, "00000000-0000-0000-0000-000000000001");
        assert_eq!(item.deleted, true);
    }

    #[tokio::test]
    async fn test_get_all_items() {
        init();
        let mut mock = MockItemRepository::new();
        let uuid = Uuid::new_v4();
        let seq = 1;
        let expected_results = create_item(uuid, seq);
        let item_row = ItemRow::from(expected_results.clone());
        mock.expect_get_all_items()
            .times(1)
            .returning(move |_| {
                let cloned_row = item_row.clone();
                Box::pin(async move { Ok(vec![cloned_row]) })
            });
        let service = ItemServiceImpl::new(Arc::new(mock));
        let result = service.get_all_items(None).await;
        assert!(result.is_ok());
        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        let item = &items[0];
        assert_eq!(item.id, expected_results.id);
        assert_eq!(item.name, expected_results.name);
        assert_eq!(item.description, expected_results.description);
        assert_eq!(item.unit_price, expected_results.unit_price.to_f64().unwrap());
        assert_eq!(item.audit_info.created_by, expected_results.audit_info.created_by);
        assert_eq!(item.audit_info.changed_by, expected_results.audit_info.changed_by);
    }

    #[tokio::test]
    async fn test_create_item_invalid_price() {
        init();
        let mut _mock = MockItemRepository::new();
        let item = CreateItemRequest {
            name: "item".to_string(),
            description: "item description".to_string(),
            unit_price: -10.0,
            created_by: "user".to_string(),
        };
        let item_clone = item.clone();
        let service = ItemServiceImpl::new(Arc::new(_mock));
        let result = service.create_item(item_clone).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_matches!(error, crate::inventory::services::ServiceError::InputValidationError(_));
    }

    #[tokio::test]
    async fn test_create_item_invalid_input() {
        init();
        let _mock = MockItemRepository::new();
        let item = CreateItemRequest {
            name: "".to_string(),
            description: "item description".to_string(),
            unit_price: 10.0,
            created_by: "user".to_string(),
        };
        let item_clone = item.clone();
        let service = ItemServiceImpl::new(Arc::new(_mock));
        let result = service.create_item(item_clone).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_matches!(error, InputValidationError(_));
        match error {
            InputValidationError(msg) => assert_eq!(msg, "Invalid input: name: length is lower than 3\n"),
            _ => assert!(false, "Expected InputValidationError")
        };
    }
}