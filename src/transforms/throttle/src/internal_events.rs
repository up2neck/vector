use metrics::counter;
use vector_lib::emit;
use vector_lib::internal_event::{
    error_stage, error_type, ComponentEventsDropped, InternalEvent, INTENTIONAL, UNINTENTIONAL,
};

#[derive(Debug)]
pub(crate) struct ThrottleEventDiscarded {
    pub key: String,
    pub emit_events_discarded_per_key: bool,
}

impl InternalEvent for ThrottleEventDiscarded {
    fn emit(self) {
        let message = "Rate limit exceeded.";

        debug!(message, key = self.key, internal_log_rate_limit = true);
        if self.emit_events_discarded_per_key {
            // TODO: Technically, the Component Specification states that the discarded events metric
            // must _only_ have the `intentional` tag, in addition to the core tags like
            // `component_kind`, etc, and nothing else.
            //
            // That doesn't give us the leeway to specify which throttle bucket the events are being
            // discarded for... but including the key/bucket as a tag does seem useful and so I wonder
            // if we should change the specification wording? Sort of a similar situation to the
            // `error_code` tag for the component errors metric, where it's meant to be optional and
            // only specified when relevant.
            counter!("events_discarded_total", 1, "key" => self.key); // Deprecated.
        }

        emit!(ComponentEventsDropped::<INTENTIONAL> {
            count: 1,
            reason: message
        })
    }
}

pub struct TemplateRenderingError<'a> {
    pub field: Option<&'a str>,
    pub drop_event: bool,
    pub error: template_lib::TemplateRenderingError,
}

impl<'a> InternalEvent for TemplateRenderingError<'a> {
    fn emit(self) {
        let mut msg = "Failed to render template".to_owned();
        if let Some(field) = self.field {
            use std::fmt::Write;
            _ = write!(msg, " for \"{}\"", field);
        }
        msg.push('.');

        if self.drop_event {
            error!(
                message = %msg,
                error = %self.error,
                error_type = error_type::TEMPLATE_FAILED,
                stage = error_stage::PROCESSING,
                internal_log_rate_limit = true,
            );

            counter!(
                "component_errors_total", 1,
                "error_type" => error_type::TEMPLATE_FAILED,
                "stage" => error_stage::PROCESSING,
            );

            emit!(ComponentEventsDropped::<UNINTENTIONAL> {
                count: 1,
                reason: "Failed to render template.",
            });
        } else {
            warn!(
                message = %msg,
                error = %self.error,
                error_type = error_type::TEMPLATE_FAILED,
                stage = error_stage::PROCESSING,
                internal_log_rate_limit = true,
            );
        }
    }
}
