use super::InternalEvent;
use crate::event::Lookup;
use metrics::counter;

#[derive(Debug)]
pub struct RenameFieldsEventProcessed;

impl InternalEvent for RenameFieldsEventProcessed {
    fn emit_metrics(&self) {
        counter!("events_processed_total", 1);
    }
}

#[derive(Debug)]
pub struct RenameFieldsFieldOverwritten<'a> {
    pub field: Lookup<'a>,
}

impl<'a> InternalEvent for RenameFieldsFieldOverwritten<'a> {
    fn emit_logs(&self) {
        debug!(message = "Field overwritten.", field = %self.field, rate_limit_secs = 30);
    }
}

#[derive(Debug)]
pub struct RenameFieldsFieldDoesNotExist<'a> {
    pub field: Lookup<'a>,
}

impl<'a> InternalEvent for RenameFieldsFieldDoesNotExist<'a> {
    fn emit_logs(&self) {
        warn!(message = "Field did not exist.", field = %self.field, rate_limit_secs = 30);
    }
}
