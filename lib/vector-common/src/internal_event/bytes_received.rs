use metrics::{counter, Counter};
use super::{ByteSize, Protocol, SharedString, InternalEventHandle};

crate::registered_event!(
    BytesReceived {
        protocol: SharedString,
    } => {
        received_bytes: Counter = counter!("component_received_bytes_total", "protocol" => self.protocol.clone()),
        protocol: SharedString = self.protocol,
    }

    fn emit(&self, data: ByteSize) {
        self.received_bytes.increment(data.0 as u64);
        trace!(message = "Bytes received.", byte_size = %data.0, protocol = %self.protocol);
    }

    fn emit_zero_value(&self) {
        self.received_bytes.increment(0);
    }
);

impl From<Protocol> for BytesReceived {
    fn from(protocol: Protocol) -> Self {
        Self {
            protocol: protocol.0,
        }
    }
}
