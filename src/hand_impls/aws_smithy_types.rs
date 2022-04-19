use aws_smithy_types::timeout::{Api, Http, Tcp};

use crate::Debug;

// These timeouts appear to have inaccessable fields

impl Debug for Http {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Http")
            .field("connect", &self.connect_timeout())
            // .field("write", &self.write_timeout())
            .field("read", &self.read_timeout())
            // .field("tls_negotiation", &self.tls_negotiation_timeout())
            .finish()
    }
}

impl Debug for Api {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Api")
            .field("call", &self.call_timeout())
            .field("call_attempt", &self.call_attempt_timeout())
            .finish()
    }
}

impl Debug for Tcp {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Tcp").finish_non_exhaustive()
    }
}

impl Debug for aws_smithy_types::retry::RetryModeParseErr {
    fn fmt(&self, f: &mut crate::Formatter) {
        // TODO: This is fine??
        f.write_debug(self)
    }
}

impl Debug for aws_smithy_types::DateTime {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("DateTime")
            .field("seconds", &self.secs())
            .field("subsecond_nanos", &self.subsec_nanos())
            .finish()
    }
}

impl Debug for aws_smithy_types::Blob {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Blob")
            .field("inner", &self.as_ref())
            .finish()
    }
}
