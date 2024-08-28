use crate::inventory::model::{
    AuditInfo, CreateInvoiceRequest, DeleteResults, Invoice, Pagination, UpdateInvoiceRequest,
};
use crate::inventory::repositories::invoice::{InvoiceRepository, InvoiceRow};
use crate::inventory::services::ServiceError;
use async_trait::async_trait;
use bigdecimal::ToPrimitive;
use std::fmt::Debug;
use uuid::Uuid;

#[async_trait]
pub trait InvoiceService: Debug {
    async fn list_all_invoices(
        &self,
        maybe_pagination: Option<Pagination>,
    ) -> Result<Vec<Invoice>, ServiceError>;
    async fn get_invoice(&self, id: Uuid) -> Result<Invoice, ServiceError>;
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
    async fn delete_invoice(&self, id: Uuid) -> Result<(), ServiceError>;
    async fn add_item_to_invoice(
        &self,
        invoice_id: Uuid,
        item_id: Uuid,
    ) -> Result<DeleteResults, ServiceError>;
    async fn remove_item_from_invoice(
        &self,
        invoice_id: Uuid,
        item_id: Uuid,
    ) -> Result<(), ServiceError>;
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
        let invoices = self.invoice_repo.get_all_invoices(maybe_pagination).await?;
        Ok(invoices.into_iter().map(|row| row.into()).collect())
    }

    // TODO - also need to retrieve items for each invoice; update the repo to return items
    async fn get_invoice(&self, id: Uuid) -> Result<Invoice, ServiceError> {
        let invoice = self.invoice_repo.get_by_uuid(id).await?;
        Ok(invoice.into())
    }

    async fn get_invoices_for_user(&self, user_id: Uuid) -> Result<Vec<Invoice>, ServiceError> {
        let invoices = self.invoice_repo.find_by_user_id(user_id).await?;
        Ok(invoices.into_iter().map(|row| row.into()).collect())
    }

    async fn get_invoices(&self) -> Result<Vec<Invoice>, ServiceError> {
        let invoices = self.invoice_repo.get_all_invoices(None).await?;
        Ok(invoices.into_iter().map(|row| row.into()).collect())
    }

    async fn create_invoice(
        &self,
        create_invoice_request: CreateInvoiceRequest,
    ) -> Result<Invoice, ServiceError> {
        let invoice = self.invoice_repo.create(create_invoice_request).await?;
        Ok(invoice.into())
    }

    async fn update_invoice(
        &self,
        update_invoice_request: UpdateInvoiceRequest,
    ) -> Result<Invoice, ServiceError> {
        let invoice = self.invoice_repo.update(update_invoice_request).await?;
        Ok(invoice.into())
    }

    async fn delete_invoice(&self, id: Uuid) -> Result<(), ServiceError> {
        self.invoice_repo.delete(id).await?;
        Ok(())
    }

    async fn add_item_to_invoice(
        &self,
        invoice_id: Uuid,
        item_id: Uuid,
    ) -> Result<DeleteResults, ServiceError> {
        unimplemented!("add_item_to_invoice not implemented")
    }

    async fn remove_item_from_invoice(
        &self,
        invoice_id: Uuid,
        item_id: Uuid,
    ) -> Result<(), ServiceError> {
        unimplemented!("remove_item_from_invoice not implemented")
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

#[cfg(test)]
mod tests {}
