use crate::inventory::model::{
    ApiError, AuditInfo, CreateItemRequest, DeleteResults, Item, Pagination, UpdateItemRequest,
};
use crate::inventory::services::ServiceError;
use crate::jwt::Claims;
use crate::AppContext;
use axum::extract::{Path, Query, State};
use axum::Json;
use tracing::instrument;
use utoipa::OpenApi;
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(
    paths(get_items, get_item_by_id, create_item, update_item, delete_item),
    components(schemas(Item, CreateItemRequest, UpdateItemRequest, ApiError, AuditInfo))
)]
pub struct ItemApi;

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
    get,
    path = "",
    summary = "Get all items",
    description = "Get all items in the inventory",
    params(
       Pagination,
       ("Authorization", Header, description="Bearer token")
    ),
    responses(
       (status = 200, description = "Items returned", body=[Item]),
       (status = 400, description = "Bad Request", body=ApiError),
       (status = 401, description = "Unauthorized", body=ApiError),
       (status = 403, description = "Forbidden", body=ApiError),
       (status = 500, description = "Internal Server Error", body=ApiError)

    )
)]
pub async fn get_items(
    claims: Claims,
    maybe_pagination_query: Option<Query<Pagination>>,
    State(app_context): State<AppContext>,
) -> Result<Json<Vec<Item>>, ServiceError> {
    let pagination = if let Some(pagination_query) = maybe_pagination_query {
        Some(pagination_query.0)
    } else {
        None
    };
    app_context
        .item_service
        .get_all_items(pagination)
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
    get,
    path = "/{id}",
    summary = "Get item by ID",
    description = "Get an item by its unique identifier (uuid)",
    params(
       ("id", Path, description="The unique identifier of the item"),
       ("Authorization", Header, description="Bearer token")
    ),
    responses(
       (status = 200, description = "Item returned", body=Item),
       (status = 400, description = "Bad Request", body=ApiError),
       (status = 401, description = "Unauthorized", body=ApiError),
       (status = 403, description = "Forbidden", body=ApiError),
       (status = 404, description = "Not Found", body=ApiError),
       (status = 500, description = "Internal Server Error", body=ApiError)
    )
)]
pub async fn get_item_by_id(
    claims: Claims,
    Path(id): Path<Uuid>,
    State(app_context): State<AppContext>,
) -> Result<Json<Item>, ServiceError> {
    app_context.item_service.get_item_by_id(id).await.map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
    post,
    path = "",
    summary = "Create an item",
    description = "Create a new item in the inventory",
    request_body = CreateItemRequest,
    params(
       ("Authorization", Header, description="Bearer token")
    ),
    responses(
       (status = 201, description = "Item created", body=Item),
       (status = 400, description = "Bad Request", body=ApiError),
       (status = 401, description = "Unauthorized", body=ApiError),
       (status = 403, description = "Forbidden", body=ApiError),
       (status = 500, description = "Internal Server Error", body=ApiError)
    )
)]
pub async fn create_item(
    claims: Claims,
    State(app_context): State<AppContext>,
    Json(create_item_request): Json<CreateItemRequest>,
) -> Result<Json<Item>, ServiceError> {
    app_context
        .item_service
        .create_item(create_item_request)
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
    put,
    path = "/{id}",
    summary = "Update an item",
    description = "Update an existing item in the inventory",
    request_body = UpdateItemRequest,
    params(
       ("id", Path, description="The unique identifier of the item"),
       ("Authorization", Header, description="Bearer token")
    ),
    responses(
       (status = 200, description = "Item updated", body=Item),
       (status = 400, description = "Bad Request", body=ApiError),
       (status = 401, description = "Unauthorized", body=ApiError),
       (status = 403, description = "Forbidden", body=ApiError),
       (status = 404, description = "Not Found", body=ApiError),
       (status = 500, description = "Internal Server Error", body=ApiError)
    )
)]
pub async fn update_item(
    claims: Claims,
    Path(id): Path<String>,
    State(app_context): State<AppContext>,
    Json(update_item_request): Json<UpdateItemRequest>,
) -> Result<Json<Item>, ServiceError> {
    if id != update_item_request.id.to_string() {
        return Err(ServiceError::InputValidationError(format!(
            "ID in path does not match ID in request. path: {}, request: {}",
            id, update_item_request.id
        )));
    }
    app_context
        .item_service
        .update_item(update_item_request)
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
    delete,
    path = "/{id}",
    summary = "Delete an item",
    description = "Delete an item from the inventory",
    params(
       ("id", Path, description="The unique identifier of the item"),
       ("Authorization", Header, description="Bearer token")
    ),
    responses(
       (status = 204, description = "Item deleted", body=()),
       (status = 400, description = "Bad Request", body=ApiError),
       (status = 401, description = "Unauthorized", body=ApiError),
       (status = 403, description = "Forbidden", body=ApiError),
       (status = 404, description = "Not Found", body=ApiError),
       (status = 500, description = "Internal Server Error", body=ApiError)
    )
)]
pub async fn delete_item(
    claims: Claims,
    Path(id): Path<Uuid>,
    State(app_context): State<AppContext>,
) -> Result<Json<DeleteResults>, ServiceError> {
    app_context.item_service.delete_item(id).await.map(Json)
}

#[cfg(test)]
mod tests {
    use crate::inventory::model::{Item, Pagination};
    use crate::inventory::services::invoice::MockInvoiceService;
    use crate::inventory::services::item::MockItemService;
    use crate::inventory::services::person::MockPersonService;
    use crate::jwt::Claims;
    use crate::test_helpers::{first_item_uuid, test_app_context, FIRST_ITEM_UUID};
    use axum::extract::{Path, Query, State};

    #[tokio::test]
    async fn test_get_items() {
        let expected_item = Item {
            seq: 1,
            id: "1".to_string(),
            name: "Item 1".to_string(),
            description: "Item 1 Description".to_string(),
            unit_price: 100.0,
            audit_info: Default::default(),
        };
        let cloned_item = expected_item.clone();
        let mut mock_item_service = MockItemService::new();
        mock_item_service
            .expect_get_all_items()
            .returning(move |_| {
                let cloned_item = cloned_item.clone();
                Box::pin(async move { Ok(vec![cloned_item]) })
            });
        let app_context = test_app_context(
            MockPersonService::new(),
            mock_item_service,
            MockInvoiceService::new(),
        );
        let no_pagination: Option<Query<Pagination>> = None;
        let result = super::get_items(Claims::default(), no_pagination, State(app_context)).await;
        assert!(result.is_ok());
        let items = result.unwrap().0;
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], expected_item);
    }

    #[tokio::test]
    async fn test_get_item_by_id() {
        let expected_item = Item {
            seq: 1,
            id: "1".to_string(),
            name: "Item 1".to_string(),
            description: "Item 1 Description".to_string(),
            unit_price: 100.0,
            audit_info: Default::default(),
        };
        let cloned_item = expected_item.clone();
        let mut mock_item_service = MockItemService::new();
        mock_item_service
            .expect_get_item_by_id()
            .returning(move |_| {
                let cloned_item = cloned_item.clone();
                Box::pin(async move { Ok(cloned_item) })
            });
        let app_context = test_app_context(
            MockPersonService::new(),
            mock_item_service,
            MockInvoiceService::new(),
        );
        let result = super::get_item_by_id(
            Claims::default(),
            Path(first_item_uuid()),
            State(app_context),
        )
        .await;
        assert!(result.is_ok());
        let item = result.unwrap().0;
        assert_eq!(item, expected_item);
    }

    #[tokio::test]
    async fn test_create_item() {
        let expected_item = Item {
            seq: 1,
            id: "1".to_string(),
            name: "Test Item".to_string(),
            description: "Test Item Description".to_string(),
            unit_price: 100.0,
            audit_info: Default::default(),
        };
        let cloned_item = expected_item.clone();
        let mut mock_item_service = MockItemService::new();
        mock_item_service.expect_create_item().returning(move |_| {
            let cloned_item = cloned_item.clone();
            Box::pin(async move { Ok(cloned_item) })
        });
        let app_context = test_app_context(
            MockPersonService::new(),
            mock_item_service,
            MockInvoiceService::new(),
        );
        let result = super::create_item(
            Claims::default(),
            State(app_context),
            axum::Json(super::CreateItemRequest {
                name: "Test Item".to_string(),
                description: "Test Item Description".to_string(),
                unit_price: 100.0,
                created_by: "testuser".to_string(),
            }),
        )
        .await;
        assert!(result.is_ok());
        let item = result.unwrap().0;
        assert_eq!(item, expected_item);
    }

    #[tokio::test]
    async fn test_update_item() {
        let expected_item = Item {
            seq: 1,
            id: FIRST_ITEM_UUID.to_string(),
            name: "Updated Item".to_string(),
            description: "Updated Item Description".to_string(),
            unit_price: 200.0,
            audit_info: Default::default(),
        };
        let cloned_item = expected_item.clone();
        let mut mock_item_service = MockItemService::new();
        mock_item_service.expect_update_item().returning(move |_| {
            let cloned_item = cloned_item.clone();
            Box::pin(async move { Ok(cloned_item) })
        });
        let app_context = test_app_context(
            MockPersonService::new(),
            mock_item_service,
            MockInvoiceService::new(),
        );
        let result = super::update_item(
            Claims::default(),
            Path(FIRST_ITEM_UUID.to_string()),
            State(app_context),
            axum::Json(super::UpdateItemRequest {
                id: first_item_uuid().to_string(),
                name: "Updated Item".to_string(),
                description: "Updated Item Description".to_string(),
                unit_price: 200.0,
                changed_by: "testuser".to_string(),
            }),
        )
        .await;
        assert!(result.is_ok());
        let item = result.unwrap().0;
        assert_eq!(item, expected_item);
    }

    #[tokio::test]
    async fn test_delete_item() {
        let expected_result = super::DeleteResults {
            id: "1".to_string(),
            deleted: true,
        };
        let cloned_result = expected_result.clone();
        let mut mock_item_service = MockItemService::new();
        mock_item_service.expect_delete_item().returning(move |_| {
            let cloned_result = cloned_result.clone();
            Box::pin(async move { Ok(cloned_result) })
        });
        let app_context = test_app_context(
            MockPersonService::new(),
            mock_item_service,
            MockInvoiceService::new(),
        );
        let result = super::delete_item(
            Claims::default(),
            Path(first_item_uuid()),
            State(app_context),
        )
        .await;
        assert!(result.is_ok());
        let delete_results = result.unwrap().0;
        assert_eq!(delete_results, expected_result);
    }

    #[tokio::test]
    async fn test_get_items_with_pagination() {
        let expected_item = Item {
            seq: 1,
            id: "1".to_string(),
            name: "Item 1".to_string(),
            description: "Item 1 Description".to_string(),
            unit_price: 100.0,
            audit_info: Default::default(),
        };
        let cloned_item = expected_item.clone();
        let mut mock_item_service = MockItemService::new();
        mock_item_service
            .expect_get_all_items()
            .returning(move |_| {
                let cloned_item = cloned_item.clone();
                Box::pin(async move { Ok(vec![cloned_item]) })
            });
        let app_context = test_app_context(
            MockPersonService::new(),
            mock_item_service,
            MockInvoiceService::new(),
        );
        let result = super::get_items(
            Claims::default(),
            Some(axum::extract::Query(super::Pagination {
                last_id: None,
                page_size: 10,
            })),
            State(app_context),
        )
        .await;
        assert!(result.is_ok());
        let items = result.unwrap().0;
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], expected_item);
    }
}
