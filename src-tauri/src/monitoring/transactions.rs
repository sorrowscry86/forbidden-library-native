//! Transaction handling for monitoring
//!
//! This module provides utilities for managing Sentry transactions.

use sentry::{start_transaction, TransactionContext, Transaction};
use std::sync::Arc;

/// A transaction that automatically finishes when dropped
pub struct ScopedTransaction {
    transaction: Option<Arc<Transaction>>,
}

impl ScopedTransaction {
    /// Create a new scoped transaction
    pub fn new(name: &str, op: &str) -> Self {
        let transaction = start_transaction(
            TransactionContext::new(name, op),
        );
        
        Self {
            transaction: Some(Arc::new(transaction)),
        }
    }
    
    /// Manually finish the transaction
    pub fn finish(&mut self) {
        if let Some(transaction) = self.transaction.take() {
            Arc::try_unwrap(transaction)
                .map(|t| t.finish())
                .ok();
        }
    }
}

impl Drop for ScopedTransaction {
    fn drop(&mut self) {
        self.finish();
    }
}

/// Create a transaction that automatically finishes when dropped
pub fn scoped_transaction(name: &str, op: &str) -> ScopedTransaction {
    ScopedTransaction::new(name, op)
}