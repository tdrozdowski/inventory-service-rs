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
