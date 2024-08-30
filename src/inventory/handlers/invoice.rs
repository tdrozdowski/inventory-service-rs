use crate::inventory::model::{
    ApiError, CreateInvoiceRequest, DeleteResults, Invoice, InvoiceItemRequest, Pagination,
    ServiceResults, UpdateInvoiceRequest, WithItemsQuery,
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
    paths(
        get_invoices,
        get_invoice_by_id,
        create_invoice,
        update_invoice,
        add_invoice_items,
        remove_invoice_item,
        delete_invoice
    ),
    components(schemas(
        Invoice,
        Pagination,
        ApiError,
        CreateInvoiceRequest,
        UpdateInvoiceRequest,
        InvoiceItemRequest,
        ServiceResults,
        DeleteResults,
        WithItemsQuery
    ))
)]
pub struct InvoiceApi;

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
   get,
   path = "",
   summary = "List all invoices",
   description = "List all invoices",
   params(
      Pagination,
      ("Authorization", Header, description = "Bearer token"),
   ),
   responses(
      (status = 200, description = "List of invoices", body=[Invoice]),
      (status = 401, description = "Unauthorized", body =ApiError),
      (status = 403, description = "Forbidden", body = ApiError),
      (status = 500, description = "Internal Server Error", body = ApiError),
   )
)]
pub async fn get_invoices(
    claims: Claims,
    maybe_pagination_query: Option<Query<Pagination>>,
    State(app_context): State<AppContext>,
) -> Result<Json<Vec<Invoice>>, ServiceError> {
    let pagination = if let Some(pagination_query) = maybe_pagination_query {
        Some(pagination_query.0)
    } else {
        None
    };
    app_context
        .invoice_service
        .list_all_invoices(pagination)
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
   get,
   path = "/{invoice_id}",
   summary = "Get an invoice by id",
   description = "Get an invoice by id (uuid)",
   params(
      ("invoice_id", Path, description = "Invoice id (uuid)"),
      ("with_items", Query, description = "Include items in response"),
      ("Authorization", Header, description = "Bearer token"),
   ),
   responses(
      (status = 200, description = "Invoice", body = Invoice),
      (status = 401, description = "Unauthorized", body = ApiError),
      (status = 403, description = "Forbidden", body = ApiError),
      (status = 404, description = "Not Found", body = ApiError),
      (status = 500, description = "Internal Server Error", body = ApiError),
   )
)]
pub async fn get_invoice_by_id(
    claims: Claims,
    Path(invoice_id): Path<Uuid>,
    with_items: Option<Query<WithItemsQuery>>,
    State(app_context): State<AppContext>,
) -> Result<Json<Invoice>, ServiceError> {
    let with_items = if let Some(with_items_query) = with_items {
        with_items_query.0.with_items
    } else {
        false
    };
    app_context
        .invoice_service
        .get_invoice(invoice_id, with_items)
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
   post,
   path = "",
   summary = "Create an invoice",
   description = "Create an invoice",
   params(
      ("Authorization", Header, description = "Bearer token"),
   ),
   request_body = CreateInvoiceRequest,
   responses(
      (status = 201, description = "Invoice created", body = Invoice),
      (status = 400, description = "Bad Request", body = ApiError),
      (status = 401, description = "Unauthorized", body = ApiError),
      (status = 403, description = "Forbidden", body = ApiError),
      (status = 500, description = "Internal Server Error", body = ApiError),
   )
)]
pub async fn create_invoice(
    claims: Claims,
    State(app_context): State<AppContext>,
    Json(invoice): Json<CreateInvoiceRequest>,
) -> Result<Json<Invoice>, ServiceError> {
    app_context
        .invoice_service
        .create_invoice(invoice)
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
   put,
    path = "/{invoice_id}",
    summary = "Update an invoice",
    description = "Update an invoice",
    params(
        ("invoice_id", Path, description = "Invoice id (uuid)"),
        ("Authorization", Header, description = "Bearer token"),
    ),
    request_body = UpdateInvoiceRequest,
    responses(
        (status = 200, description = "Invoice updated", body = Invoice),
        (status = 400, description = "Bad Request", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 404, description = "Not Found", body = ApiError),
        (status = 500, description = "Internal Server Error", body = ApiError),
    )
)]
pub async fn update_invoice(
    claims: Claims,
    Path(invoice_id): Path<String>,
    State(app_context): State<AppContext>,
    Json(invoice): Json<UpdateInvoiceRequest>,
) -> Result<Json<Invoice>, ServiceError> {
    if invoice_id != invoice.id.to_string() {
        return Err(ServiceError::InputValidationError(format!(
            "Invoice id in path ({}) does not match id in body ({})",
            invoice_id, invoice.id
        )));
    }
    app_context
        .invoice_service
        .update_invoice(invoice)
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
   post,
   path = "/{invoice_id}/items",
   summary = "Add items to an invoice",
   description = "Add items to an invoice",
   params(
      ("invoice_id", Path, description = "Invoice id (uuid)"),
      ("Authorization", Header, description = "Bearer token"),
   ),
   request_body = InvoiceItemRequest,
   responses(
      (status = 200, description = "Items added", body = ServiceResults),
      (status = 400, description = "Bad Request", body = ApiError),
      (status = 401, description = "Unauthorized", body = ApiError),
      (status = 403, description = "Forbidden", body = ApiError),
      (status = 404, description = "Not Found", body = ApiError),
      (status = 500, description = "Internal Server Error", body = ApiError),
   )
)]
pub async fn add_invoice_items(
    claims: Claims,
    Path(invoice_id): Path<String>,
    State(app_context): State<AppContext>,
    Json(invoice_item_request): Json<InvoiceItemRequest>,
) -> Result<Json<ServiceResults>, ServiceError> {
    if invoice_id != invoice_item_request.invoice_id.to_string() {
        return Err(ServiceError::InputValidationError(format!(
            "Invoice id in path ({}) does not match id in body ({})",
            invoice_id, invoice_item_request.invoice_id
        )));
    }
    app_context
        .invoice_service
        .add_item_to_invoice(
            invoice_item_request.invoice_id,
            invoice_item_request.item_id,
        )
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
   delete,
   path = "/{invoice_id}/items/{item_id}",
   summary = "Remove an item from an invoice",
   description = "Remove an item from an invoice",
   params(
      ("invoice_id", Path, description = "Invoice id (uuid)"),
      ("item_id", Path, description = "Item id (uuid)"),
      ("Authorization", Header, description = "Bearer token"),
   ),
   responses(
      (status = 200, description = "Item removed", body = DeleteResults),
      (status = 400, description = "Bad Request", body = ApiError),
      (status = 401, description = "Unauthorized", body = ApiError),
      (status = 403, description = "Forbidden", body = ApiError),
      (status = 404, description = "Not Found", body = ApiError),
      (status = 500, description = "Internal Server Error", body = ApiError),
   )
)]
pub async fn remove_invoice_item(
    claims: Claims,
    Path(invoice_item): Path<InvoiceItemRequest>,
    State(app_context): State<AppContext>,
) -> Result<Json<DeleteResults>, ServiceError> {
    app_context
        .invoice_service
        .remove_item_from_invoice(invoice_item.invoice_id, invoice_item.item_id)
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
    delete,
    path = "/{invoice_id}",
    summary = "Delete an invoice",
    description = "Delete an invoice",
    params(
        ("invoice_id", Path, description = "Invoice id (uuid)"),
        ("Authorization", Header, description = "Bearer token"),
    ),
    responses(
        (status = 200, description = "Invoice deleted", body = DeleteResults),
        (status = 400, description = "Bad Request", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 404, description = "Not Found", body = ApiError),
        (status = 500, description = "Internal Server Error", body = ApiError),
    )
)]
pub async fn delete_invoice(
    claims: Claims,
    Path(invoice_id): Path<String>,
    State(app_context): State<AppContext>,
) -> Result<Json<DeleteResults>, ServiceError> {
    let invoice_id = Uuid::parse_str(&invoice_id).map_err(|_| {
        ServiceError::InputValidationError(format!("Invalid invoice id: {}", invoice_id))
    })?;
    app_context
        .invoice_service
        .delete_invoice(invoice_id)
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
    get,
    path = "/users/{user_id}",
    summary = "List all invoices for user",
    description = "List all invoices for user",
    params(
        ("user_id", Path, description = "User id (uuid)"),
        ("Authorization", Header, description = "Bearer token"),
    ),
    responses(
        (status = 200, description = "List of invoices", body = [Invoice]),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 403, description = "Forbidden", body = ApiError),
        (status = 404, description = "Not Found", body = ApiError),
        (status = 500, description = "Internal Server Error", body = ApiError),
    )
)]
pub async fn get_invoices_by_user(
    claims: Claims,
    Path(user_id): Path<Uuid>,
    State(app_context): State<AppContext>,
) -> Result<Json<Vec<Invoice>>, ServiceError> {
    app_context
        .invoice_service
        .get_invoices_for_user(user_id)
        .await
        .map(Json)
}

#[cfg(test)]
mod tests {
    use crate::inventory::handlers::invoice::{
        add_invoice_items, get_invoice_by_id, get_invoices, get_invoices_by_user, update_invoice,
    };
    use crate::inventory::model::{
        CreateInvoiceRequest, DeleteResults, Invoice, ServiceResults, WithItemsQuery,
    };
    use crate::inventory::services::invoice::MockInvoiceService;
    use crate::inventory::services::item::MockItemService;
    use crate::inventory::services::person::MockPersonService;
    use crate::inventory::services::ServiceError::NotFound;
    use crate::test_helpers::{mock_claims, test_app_context};
    use axum::extract::{Path, Query, State};
    use uuid::Uuid;

    fn create_invoice(item_id: Uuid) -> Invoice {
        Invoice {
            seq: 1,
            id: item_id.to_string(),
            user_id: Uuid::new_v4().to_string(),
            total: 0.0,
            items: vec![],
            audit_info: Default::default(),
            paid: false,
        }
    }

    fn create_invoice_with_items(item_id: Uuid) -> Invoice {
        Invoice {
            seq: 1,
            id: item_id.to_string(),
            user_id: Uuid::new_v4().to_string(),
            total: 0.0,
            items: vec![Default::default()],
            audit_info: Default::default(),
            paid: false,
        }
    }

    #[tokio::test]
    async fn test_get_invoices() {
        let expected_invoice = create_invoice(Uuid::new_v4());
        let cloned_invoice = expected_invoice.clone();
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_list_all_invoices()
            .returning(move |_| {
                let cloned_invoice = cloned_invoice.clone();
                Box::pin(async move { Ok(vec![cloned_invoice]) })
            });
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let response = get_invoices(claims, None, State(app_context)).await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.0.len(), 1);
    }

    #[tokio::test]
    async fn test_get_invoice_by_id() {
        let expected_invoice = create_invoice(Uuid::new_v4());
        let cloned_invoice = expected_invoice.clone();
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_get_invoice()
            .returning(move |_, _| {
                let cloned_invoice = cloned_invoice.clone();
                Box::pin(async move { Ok(cloned_invoice) })
            });
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let response =
            get_invoice_by_id(claims, Path(Uuid::new_v4()), None, State(app_context)).await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.0.id, expected_invoice.id);
        assert_eq!(response.0.items.len(), 0);
    }

    #[tokio::test]
    async fn test_get_invoice_by_id_with_items() {
        let expected_invoice = create_invoice_with_items(Uuid::new_v4());
        let cloned_invoice = expected_invoice.clone();
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_get_invoice()
            .returning(move |_, _| {
                let cloned_invoice = cloned_invoice.clone();
                Box::pin(async move { Ok(cloned_invoice) })
            });
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let item_query = WithItemsQuery { with_items: true };
        let response = get_invoice_by_id(
            claims,
            Path(Uuid::new_v4()),
            Some(Query(item_query)),
            State(app_context),
        )
        .await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.0.id, expected_invoice.id);
        assert_eq!(response.0.items.len(), 1);
    }

    #[tokio::test]
    async fn test_get_invoice_by_id_with_items_false() {
        let expected_invoice = create_invoice(Uuid::new_v4());
        let cloned_invoice = expected_invoice.clone();
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_get_invoice()
            .returning(move |_, _| {
                let cloned_invoice = cloned_invoice.clone();
                Box::pin(async move { Ok(cloned_invoice) })
            });
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let item_query = WithItemsQuery { with_items: false };
        let response = get_invoice_by_id(
            claims,
            Path(Uuid::new_v4()),
            Some(Query(item_query)),
            State(app_context),
        )
        .await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.0.id, expected_invoice.id);
        assert_eq!(response.0.items.len(), 0);
    }

    #[tokio::test]
    async fn test_create_invoice() {
        let expected_invoice = create_invoice(Uuid::new_v4());
        let cloned_invoice = expected_invoice.clone();
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_create_invoice()
            .returning(move |_| {
                let cloned_invoice = cloned_invoice.clone();
                Box::pin(async move { Ok(cloned_invoice) })
            });
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let create_request = CreateInvoiceRequest {
            user_id: Uuid::new_v4(),
            total: 0.0,
            created_by: "unit_test".to_string(),
            items: vec![],
            paid: false,
        };
        let response = crate::inventory::handlers::invoice::create_invoice(
            claims,
            State(app_context),
            axum::Json(create_request),
        )
        .await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.0.id, expected_invoice.id);
    }

    #[tokio::test]
    async fn test_update_invoice() {
        let expected_invoice = create_invoice(Uuid::new_v4());
        let cloned_invoice = expected_invoice.clone();
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_update_invoice()
            .returning(move |_| {
                let cloned_invoice = cloned_invoice.clone();
                Box::pin(async move { Ok(cloned_invoice) })
            });
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let update_request = crate::inventory::model::UpdateInvoiceRequest {
            id: Uuid::parse_str(expected_invoice.id.as_str()).unwrap(),
            total: 0.0,
            changed_by: "unit_test".to_string(),
            paid: false,
        };
        let response = update_invoice(
            claims,
            Path(expected_invoice.id.clone()),
            State(app_context),
            axum::Json(update_request),
        )
        .await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.0.id, expected_invoice.id);
    }

    #[tokio::test]
    async fn test_add_invoice_items() {
        let expected_results = ServiceResults {
            success: true,
            message: "Items added".to_string(),
        };
        let cloned_results = expected_results.clone();
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_add_item_to_invoice()
            .returning(move |_, _| {
                let cloned_results = cloned_results.clone();
                Box::pin(async move { Ok(cloned_results) })
            });
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let item_request = crate::inventory::model::InvoiceItemRequest {
            invoice_id: Uuid::new_v4(),
            item_id: Uuid::new_v4(),
        };
        let response = add_invoice_items(
            claims,
            Path(item_request.invoice_id.to_string()),
            State(app_context),
            axum::Json(item_request),
        )
        .await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(response.0.success);
    }

    #[tokio::test]
    async fn test_remove_invoice_item() {
        let invoice_id = Uuid::new_v4();
        let cloned_invoice_id = invoice_id.clone();
        let expected_results = DeleteResults {
            id: invoice_id.to_string(),
            deleted: true,
        };
        let cloned_results = expected_results.clone();
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_remove_item_from_invoice()
            .returning(move |_, _| {
                let cloned_results = cloned_results.clone();
                Box::pin(async move { Ok(cloned_results) })
            });
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let item_request = crate::inventory::model::InvoiceItemRequest {
            invoice_id: cloned_invoice_id,
            item_id: Uuid::new_v4(),
        };
        let response = crate::inventory::handlers::invoice::remove_invoice_item(
            claims,
            Path(item_request),
            State(app_context),
        )
        .await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(response.0.deleted);
    }

    #[tokio::test]
    async fn test_delete_invoice() {
        let invoice_id = Uuid::new_v4();
        let expected_results = DeleteResults {
            id: invoice_id.to_string(),
            deleted: true,
        };
        let cloned_results = expected_results.clone();
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_delete_invoice()
            .returning(move |_| {
                let cloned_results = cloned_results.clone();
                Box::pin(async move { Ok(cloned_results) })
            });
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let response = crate::inventory::handlers::invoice::delete_invoice(
            claims,
            Path(invoice_id.to_string()),
            State(app_context),
        )
        .await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(response.0.deleted);
    }

    #[tokio::test]
    async fn test_add_item_to_invoice_mismatch_invoice_id() {
        let invoice_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();
        let item_request = crate::inventory::model::InvoiceItemRequest {
            invoice_id,
            item_id,
        };
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            MockInvoiceService::new(),
        );
        let claims = mock_claims();
        let response = crate::inventory::handlers::invoice::add_invoice_items(
            claims,
            Path(Uuid::new_v4().to_string()),
            State(app_context),
            axum::Json(item_request),
        )
        .await;
        assert!(response.is_err());
        let response = response.unwrap_err();
        assert!(matches!(
            response,
            crate::inventory::services::ServiceError::InputValidationError(_)
        ));
    }

    #[tokio::test]
    async fn test_update_invoice_mismatch_invoice_id() {
        let update_request = crate::inventory::model::UpdateInvoiceRequest {
            id: Uuid::new_v4(),
            total: 0.0,
            changed_by: "unit_test".to_string(),
            paid: false,
        };
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            MockInvoiceService::new(),
        );
        let claims = mock_claims();
        let response = update_invoice(
            claims,
            Path(Uuid::new_v4().to_string()),
            State(app_context),
            axum::Json(update_request),
        )
        .await;
        assert!(response.is_err());
        let response = response.unwrap_err();
        assert!(matches!(
            response,
            crate::inventory::services::ServiceError::InputValidationError(_)
        ));
    }

    #[tokio::test]
    async fn test_get_invoice_by_id_not_found() {
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_get_invoice()
            .returning(move |_, _| Box::pin(async move { Err(NotFound("".to_string())) }));
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let response =
            get_invoice_by_id(claims, Path(Uuid::new_v4()), None, State(app_context)).await;
        assert!(response.is_err());
        let response = response.unwrap_err();
        match response {
            NotFound(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_remove_invoice_item_not_found() {
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_remove_item_from_invoice()
            .returning(move |_, _| Box::pin(async move { Err(NotFound("".to_string())) }));
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let item_request = crate::inventory::model::InvoiceItemRequest {
            invoice_id: Uuid::new_v4(),
            item_id: Uuid::new_v4(),
        };
        let response = crate::inventory::handlers::invoice::remove_invoice_item(
            claims,
            Path(item_request),
            State(app_context),
        )
        .await;
        assert!(response.is_err());
        let response = response.unwrap_err();
        match response {
            NotFound(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_delete_invoice_not_found() {
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_delete_invoice()
            .returning(move |_| Box::pin(async move { Err(NotFound("".to_string())) }));
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let response = crate::inventory::handlers::invoice::delete_invoice(
            claims,
            Path(Uuid::new_v4().to_string()),
            State(app_context),
        )
        .await;
        assert!(response.is_err());
        let response = response.unwrap_err();
        match response {
            NotFound(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_add_invoice_items_not_found() {
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_add_item_to_invoice()
            .returning(move |_, _| Box::pin(async move { Err(NotFound("".to_string())) }));
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let item_request = crate::inventory::model::InvoiceItemRequest {
            invoice_id: Uuid::new_v4(),
            item_id: Uuid::new_v4(),
        };
        let response = crate::inventory::handlers::invoice::add_invoice_items(
            claims,
            Path(item_request.invoice_id.to_string()),
            State(app_context),
            axum::Json(item_request),
        )
        .await;
        assert!(response.is_err());
        let response = response.unwrap_err();
        match response {
            NotFound(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_get_invoices_by_user() {
        let expected_invoice = create_invoice(Uuid::new_v4());
        let cloned_invoice = expected_invoice.clone();
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_get_invoices_for_user()
            .returning(move |_| {
                let cloned_invoice = cloned_invoice.clone();
                Box::pin(async move { Ok(vec![cloned_invoice]) })
            });
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let response = get_invoices_by_user(claims, Path(Uuid::new_v4()), State(app_context)).await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.0.len(), 1);
    }

    #[tokio::test]
    async fn test_get_invoices_by_user_not_found() {
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_get_invoices_for_user()
            .returning(move |_| Box::pin(async move { Err(NotFound("".to_string())) }));
        let app_context = test_app_context(
            MockPersonService::new(),
            MockItemService::new(),
            mock_invoice_service,
        );
        let claims = mock_claims();
        let response = get_invoices_by_user(claims, Path(Uuid::new_v4()), State(app_context)).await;
        assert!(response.is_err());
        let response = response.unwrap_err();
        match response {
            NotFound(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
