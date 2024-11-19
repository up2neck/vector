use metrics::{counter, Counter};
use tracing::trace;
use super::{ByteSize, Protocol, SharedString, InternalEventHandle};

crate::registered_event!(
    BytesSent {
        protocol: SharedString,
    } => {
        bytes_sent: Counter = counter!("component_sent_bytes_total", "protocol" => self.protocol.clone()),
        protocol: SharedString = self.protocol,
    }

    fn emit(&self, byte_size: ByteSize) {
        trace!(message = "Bytes sent.", byte_size = %byte_size.0, protocol = %self.protocol);
        self.bytes_sent.increment(byte_size.0 as u64);
    }

    fn emit_zero_value(&self) {
        self.bytes_sent.increment(0);
    }
);

impl From<Protocol> for BytesSent {
    fn from(protocol: Protocol) -> Self {
        Self {
            protocol: protocol.0,
        }
    }
}
