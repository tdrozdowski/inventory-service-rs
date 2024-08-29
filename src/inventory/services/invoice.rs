use crate::inventory::model::{
    AuditInfo, CreateInvoiceRequest, DeleteResults, Invoice, Item, Pagination, ServiceResults,
    UpdateInvoiceRequest,
};
use crate::inventory::repositories::invoice::{
    InvoiceItemRow, InvoiceRepository, InvoiceRow, InvoiceWithItemRow,
};
use crate::inventory::services::ServiceError;
use async_trait::async_trait;
use bigdecimal::ToPrimitive;
use mockall::automock;
use std::fmt::Debug;
use uuid::Uuid;

#[async_trait]
#[automock]
pub trait InvoiceService: Debug {
    async fn list_all_invoices(
        &self,
        maybe_pagination: Option<Pagination>,
    ) -> Result<Vec<Invoice>, ServiceError>;
    async fn get_invoice(&self, id: Uuid, with_items: bool) -> Result<Invoice, ServiceError>;
    async fn get_invoices_for_user(&self, user_id: Uuid) -> Result<Vec<Invoice>, ServiceError>;
    async fn get_invoices(&self) -> Result<Vec<Invoice>, ServiceError>;
    async fn create_invoice(
        &self,
        create_invoice_request: CreateInvoiceRequest,
    ) -> Result<Invoice, ServiceError>;
    async fn update_invoice(
        &self,
        update_invoice_request: UpdateInvoiceRequest,
    ) -> Result<Invoice, ServiceError>;
    async fn delete_invoice(&self, id: Uuid) -> Result<DeleteResults, ServiceError>;
    async fn add_item_to_invoice(
        &self,
        invoice_id: Uuid,
        item_id: Uuid,
    ) -> Result<ServiceResults, ServiceError>;
    async fn remove_item_from_invoice(
        &self,
        invoice_id: Uuid,
        item_id: Uuid,
    ) -> Result<DeleteResults, ServiceError>;
}

#[derive(Debug)]
pub struct InvoiceServiceImpl {
    invoice_repo: Box<dyn InvoiceRepository + Send + Sync>,
}

impl InvoiceServiceImpl {
    pub async fn new(invoice_repo: Box<dyn InvoiceRepository + Send + Sync>) -> Self {
        Self { invoice_repo }
    }
}

#[async_trait]
impl InvoiceService for InvoiceServiceImpl {
    async fn list_all_invoices(
        &self,
        maybe_pagination: Option<Pagination>,
    ) -> Result<Vec<Invoice>, ServiceError> {
        let results = self.invoice_repo.get_all_invoices(maybe_pagination).await;
        results
            .map(|i| i.into_iter().map(Invoice::from).collect())
            .map_err(ServiceError::from)
    }
    async fn get_invoice(&self, id: Uuid, with_items: bool) -> Result<Invoice, ServiceError> {
        if with_items {
            let results = self.invoice_repo.get_with_items(id).await;
            results.map(Invoice::from).map_err(ServiceError::from)
        } else {
            let results = self.invoice_repo.get_by_uuid(id).await;
            results.map(Invoice::from).map_err(ServiceError::from)
        }
    }

    async fn get_invoices_for_user(&self, user_id: Uuid) -> Result<Vec<Invoice>, ServiceError> {
        let results = self.invoice_repo.find_by_user_id(user_id).await;
        results
            .map(|i| i.into_iter().map(Invoice::from).collect())
            .map_err(ServiceError::from)
    }

    async fn get_invoices(&self) -> Result<Vec<Invoice>, ServiceError> {
        let results = self.invoice_repo.get_all_invoices(None).await;
        results
            .map(|i| i.into_iter().map(Invoice::from).collect())
            .map_err(ServiceError::from)
    }

    async fn create_invoice(
        &self,
        create_invoice_request: CreateInvoiceRequest,
    ) -> Result<Invoice, ServiceError> {
        let results = self.invoice_repo.create(create_invoice_request).await;
        results.map(Invoice::from).map_err(ServiceError::from)
    }

    async fn update_invoice(
        &self,
        update_invoice_request: UpdateInvoiceRequest,
    ) -> Result<Invoice, ServiceError> {
        let results = self.invoice_repo.update(update_invoice_request).await;
        results.map(Invoice::from).map_err(ServiceError::from)
    }

    async fn delete_invoice(&self, id: Uuid) -> Result<DeleteResults, ServiceError> {
        let results = self.invoice_repo.delete(id).await;
        results.map_err(ServiceError::from)
    }

    async fn add_item_to_invoice(
        &self,
        invoice_id: Uuid,
        item_id: Uuid,
    ) -> Result<ServiceResults, ServiceError> {
        let row = InvoiceItemRow {
            invoice_id,
            item_id,
        };
        let results = self.invoice_repo.add_item(row).await;
        results
            .map(|_| ServiceResults {
                message: format!("Item {} added to invoice {}", item_id, invoice_id),
                success: true,
            })
            .map_err(ServiceError::from)
    }

    async fn remove_item_from_invoice(
        &self,
        invoice_id: Uuid,
        item_id: Uuid,
    ) -> Result<DeleteResults, ServiceError> {
        let row = InvoiceItemRow {
            invoice_id,
            item_id,
        };
        let results = self.invoice_repo.remove_item(row).await;
        results.map_err(ServiceError::from)
    }
}

impl From<InvoiceRow> for Invoice {
    fn from(row: InvoiceRow) -> Self {
        Invoice {
            seq: row.id,
            id: row.alt_id.to_string(),
            user_id: row.user_id.to_string(),
            total: row.total.to_f64().unwrap(),
            paid: row.paid,
            audit_info: AuditInfo {
                created_by: row.created_by,
                created_at: row.created_at,
                changed_by: row.last_changed_by,
                updated_at: row.last_update,
            },
            items: vec![],
        }
    }
}

impl From<Vec<InvoiceWithItemRow>> for Invoice {
    fn from(rows: Vec<InvoiceWithItemRow>) -> Self {
        let items: Vec<Item> = rows
            .iter()
            .map(|row| Item {
                seq: 0,
                id: row.item_alt_id.to_string(),
                name: row.item_name.clone(),
                description: row.item_description.clone(),
                unit_price: row.item_unit_price.to_f64().unwrap(),
                audit_info: AuditInfo::default(),
            })
            .collect();
        let row = rows.first().unwrap();
        Invoice {
            seq: row.id,
            id: row.alt_id.to_string(),
            user_id: row.user_id.to_string(),
            total: row.total.to_f64().unwrap(),
            paid: row.paid,
            audit_info: AuditInfo {
                created_by: row.created_by.clone(),
                created_at: row.created_at,
                changed_by: row.last_changed_by.clone(),
                updated_at: row.last_update,
            },
            items: items.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::repositories::invoice::MockInvoiceRepository;
    use mockall::predicate::*;

    fn create_invoice_row(uuid: Uuid, user_id: Uuid) -> InvoiceRow {
        InvoiceRow {
            id: 1,
            alt_id: uuid,
            user_id,
            total: bigdecimal::BigDecimal::from(100),
            paid: false,
            created_by: "testuser".to_string(),
            created_at: chrono::Utc::now(),
            last_changed_by: "testuser".to_string(),
            last_update: chrono::Utc::now(),
        }
    }
    #[tokio::test]
    async fn test_list_all_invoices() {
        let mut mock = MockInvoiceRepository::new();
        let no_pagination: Option<Pagination> = None;
        let expected_row = create_invoice_row(Uuid::new_v4(), Uuid::new_v4());
        mock.expect_get_all_invoices()
            .with(eq(no_pagination))
            .times(1)
            .returning(move |_| {
                let cloned_row = expected_row.clone();
                Box::pin(async move { Ok(vec![cloned_row]) })
            });

        let service = InvoiceServiceImpl::new(Box::new(mock)).await;
        let result = service.list_all_invoices(None).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_get_all_invoices_with_pagination() {
        let mut mock = MockInvoiceRepository::new();
        let expected_row = create_invoice_row(Uuid::new_v4(), Uuid::new_v4());
        let pagination = Pagination {
            last_id: None,
            page_size: 10,
        };
        mock.expect_get_all_invoices()
            .with(eq(Some(pagination)))
            .times(1)
            .returning(move |_| {
                let cloned_row = expected_row.clone();
                Box::pin(async move { Ok(vec![cloned_row]) })
            });

        let service = InvoiceServiceImpl::new(Box::new(mock)).await;
        let result = service.list_all_invoices(Some(pagination)).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_get_invoice() {
        let mut mock = MockInvoiceRepository::new();
        let id = Uuid::new_v4();
        let expected_row = create_invoice_row(id, Uuid::new_v4());
        mock.expect_get_by_uuid()
            .with(eq(id))
            .times(1)
            .returning(move |_| {
                let cloned_row = expected_row.clone();
                Box::pin(async move { Ok(cloned_row) })
            });

        let service = InvoiceServiceImpl::new(Box::new(mock)).await;
        let result = service.get_invoice(id, false).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, id.to_string());
    }

    #[tokio::test]
    async fn test_get_invoice_with_items() {
        let mut mock = MockInvoiceRepository::new();
        let id = Uuid::new_v4();
        let expected_item = InvoiceWithItemRow {
            id: 1,
            alt_id: id,
            user_id: Uuid::new_v4(),
            total: bigdecimal::BigDecimal::from(100),
            paid: false,
            created_by: "testuser".to_string(),
            created_at: chrono::Utc::now(),
            last_changed_by: "testuser".to_string(),
            last_update: chrono::Utc::now(),
            item_alt_id: Uuid::new_v4(),
            item_name: "Test Item".to_string(),
            item_description: "Test Description".to_string(),
            item_unit_price: bigdecimal::BigDecimal::from(100),
        };
        mock.expect_get_with_items()
            .with(eq(id))
            .times(1)
            .returning(move |_| {
                let cloned_row = expected_item.clone();
                Box::pin(async move { Ok(vec![cloned_row]) })
            });

        let service = InvoiceServiceImpl::new(Box::new(mock)).await;
        let result = service.get_invoice(id, true).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, id.to_string());
    }

    #[tokio::test]
    async fn test_get_invoice_no_items() {
        let mut mock = MockInvoiceRepository::new();
        let id = Uuid::new_v4();
        let expected_row = create_invoice_row(id, Uuid::new_v4());
        mock.expect_get_by_uuid()
            .with(eq(id))
            .times(1)
            .returning(move |_| {
                let cloned_row = expected_row.clone();
                Box::pin(async move { Ok(cloned_row) })
            });

        let service = InvoiceServiceImpl::new(Box::new(mock)).await;
        let result = service.get_invoice(id, false).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, id.to_string());
    }

    #[tokio::test]
    async fn test_get_invoices_for_user() {
        let mut mock = MockInvoiceRepository::new();
        let user_id = Uuid::new_v4();
        let expected_row = create_invoice_row(Uuid::new_v4(), user_id);
        mock.expect_find_by_user_id()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| {
                let cloned_row = expected_row.clone();
                Box::pin(async move { Ok(vec![cloned_row]) })
            });

        let service = InvoiceServiceImpl::new(Box::new(mock)).await;
        let result = service.get_invoices_for_user(user_id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_create_invoice() {
        let mut mock = MockInvoiceRepository::new();
        let id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let expected_row = create_invoice_row(id, user_id);
        mock.expect_create()
            .withf(move |r| r.user_id == user_id)
            .times(1)
            .returning(move |_| {
                let cloned_row = expected_row.clone();
                Box::pin(async move { Ok(cloned_row) })
            });

        let service = InvoiceServiceImpl::new(Box::new(mock)).await;
        let result = service
            .create_invoice(CreateInvoiceRequest {
                user_id,
                total: 100.0,
                paid: false,
                created_by: "testuser".to_string(),
                items: vec![],
            })
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, id.to_string());
    }

    #[tokio::test]
    async fn test_update_invoice() {
        let mut mock = MockInvoiceRepository::new();
        let id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let expected_row = create_invoice_row(id, user_id);
        mock.expect_update()
            .withf(move |r| r.id == id)
            .times(1)
            .returning(move |_| {
                let cloned_row = expected_row.clone();
                Box::pin(async move { Ok(cloned_row) })
            });

        let service = InvoiceServiceImpl::new(Box::new(mock)).await;
        let result = service
            .update_invoice(UpdateInvoiceRequest {
                id,
                total: 100.0,
                paid: false,
                changed_by: "testuser".to_string(),
            })
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, id.to_string());
    }

    #[tokio::test]
    async fn test_delete_invoice() {
        let mut mock = MockInvoiceRepository::new();
        let id = Uuid::new_v4();
        mock.expect_delete()
            .with(eq(id))
            .times(1)
            .returning(move |_| {
                let cloned_id = id.clone().to_string();
                Box::pin(async {
                    Ok(DeleteResults {
                        id: cloned_id,
                        deleted: true,
                    })
                })
            });

        let service = InvoiceServiceImpl::new(Box::new(mock)).await;
        let result = service.delete_invoice(id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_item_to_invoice() {
        let mut mock = MockInvoiceRepository::new();
        let invoice_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();
        mock.expect_add_item()
            .withf(move |r| r.invoice_id == invoice_id && r.item_id == item_id)
            .times(1)
            .returning(move |_| {
                let cloned_invoice_id = invoice_id.clone();
                let cloned_item_id = item_id.clone();
                Box::pin(async move {
                    Ok(InvoiceItemRow {
                        invoice_id: cloned_invoice_id,
                        item_id: cloned_item_id,
                    })
                })
            });

        let service = InvoiceServiceImpl::new(Box::new(mock)).await;
        let result = service.add_item_to_invoice(invoice_id, item_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_item_from_invoice() {
        let mut mock = MockInvoiceRepository::new();
        let invoice_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();
        mock.expect_remove_item()
            .withf(move |r| r.invoice_id == invoice_id && r.item_id == item_id)
            .times(1)
            .returning(move |_| {
                let cloned_item_id = item_id.clone().to_string();
                Box::pin(async {
                    Ok(DeleteResults {
                        id: cloned_item_id,
                        deleted: true,
                    })
                })
            });

        let service = InvoiceServiceImpl::new(Box::new(mock)).await;
        let result = service.remove_item_from_invoice(invoice_id, item_id).await;
        assert!(result.is_ok());
    }
}
