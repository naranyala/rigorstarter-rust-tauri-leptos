use leptos::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

static ERROR_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub struct AppErrorMsg {
    pub id: u64,
    pub message: String,
    pub severity: ErrorSeverity,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct ErrorService {
    pub errors: RwSignal<Vec<AppErrorMsg>>,
}

impl ErrorService {
    pub fn new() -> Self {
        Self {
            errors: RwSignal::new(Vec::new()),
        }
    }

    #[allow(dead_code)]
    pub fn push(&self, message: String, severity: ErrorSeverity) {
        let id = ERROR_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        self.errors.update(|errs| {
            errs.push(AppErrorMsg {
                id,
                message,
                severity,
                timestamp: chrono::Utc::now(),
            });
        });
    }

    #[allow(dead_code)]
    pub fn clear(&self) {
        self.errors.set(Vec::new());
    }

    pub fn remove(&self, id: u64) {
        self.errors.update(|errs| {
            errs.retain(|e| e.id != id);
        });
    }
}
