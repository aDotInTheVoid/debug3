// AUTOGENERATED FILE, DO NOT EDIT
//
// Crate Name: `aws_smithy_types`
// Crate Version: `0.45.0`
// Skipping aws_smithy_types::Blob due to hidden fields
impl crate::Debug for aws_smithy_types::Document {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Object(__0) => {
                f.debug_tuple("Object").field(__0).finish();
            }
            Self::Array(__0) => {
                f.debug_tuple("Array").field(__0).finish();
            }
            Self::Number(__0) => {
                f.debug_tuple("Number").field(__0).finish();
            }
            Self::String(__0) => {
                f.debug_tuple("String").field(__0).finish();
            }
            Self::Bool(__0) => {
                f.debug_tuple("Bool").field(__0).finish();
            }
            Self::Null => {
                f.debug_tuple("Null").finish();
            }
        }
    }
}
impl crate::Debug for aws_smithy_types::Number {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::PosInt(__0) => {
                f.debug_tuple("PosInt").field(__0).finish();
            }
            Self::NegInt(__0) => {
                f.debug_tuple("NegInt").field(__0).finish();
            }
            Self::Float(__0) => {
                f.debug_tuple("Float").field(__0).finish();
            }
        }
    }
}
impl crate::Debug for aws_smithy_types::base64::DecodeError {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::InvalidByte => {
                f.debug_tuple("InvalidByte").finish();
            }
            Self::InvalidPadding => {
                f.debug_tuple("InvalidPadding").finish();
            }
            Self::InvalidLength => {
                f.debug_tuple("InvalidLength").finish();
            }
            _ => "???".fmt(f),
        }
    }
}
// Skipping aws_smithy_types::date_time::ConversionError due to hidden fields
// Skipping aws_smithy_types::date_time::DateTime due to hidden fields
impl crate::Debug for aws_smithy_types::date_time::DateTimeFormatError {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::OutOfRange(__0, ..) => {
                f.debug_tuple("OutOfRange").field(__0).finish();
            }
            _ => "???".fmt(f),
        }
    }
}
impl crate::Debug for aws_smithy_types::date_time::DateTimeParseError {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Invalid(__0, ..) => {
                f.debug_tuple("Invalid").field(__0).finish();
            }
            Self::IntParseError => {
                f.debug_tuple("IntParseError").finish();
            }
            _ => "???".fmt(f),
        }
    }
}
impl crate::Debug for aws_smithy_types::date_time::Format {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::DateTime => {
                f.debug_tuple("DateTime").finish();
            }
            Self::HttpDate => {
                f.debug_tuple("HttpDate").finish();
            }
            Self::EpochSeconds => {
                f.debug_tuple("EpochSeconds").finish();
            }
        }
    }
}
// Skipping aws_smithy_types::error::Builder due to hidden fields
// Skipping aws_smithy_types::error::Error due to hidden fields
// Skipping aws_smithy_types::primitive::Encoder due to hidden fields
// Skipping aws_smithy_types::primitive::PrimitiveParseError due to hidden
// fields
impl crate::Debug for aws_smithy_types::retry::ErrorKind {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::TransientError => {
                f.debug_tuple("TransientError").finish();
            }
            Self::ThrottlingError => {
                f.debug_tuple("ThrottlingError").finish();
            }
            Self::ServerError => {
                f.debug_tuple("ServerError").finish();
            }
            Self::ClientError => {
                f.debug_tuple("ClientError").finish();
            }
            _ => "???".fmt(f),
        }
    }
}
// Skipping aws_smithy_types::retry::RetryConfig due to hidden fields
// Skipping aws_smithy_types::retry::RetryConfigBuilder due to hidden fields
impl crate::Debug for aws_smithy_types::retry::RetryConfigErr {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::InvalidRetryMode { source, set_by } => f
                .debug_struct("InvalidRetryMode")
                .field("source", source)
                .field("set_by", set_by)
                .finish(),
            Self::MaxAttemptsMustNotBeZero { set_by } => f
                .debug_struct("MaxAttemptsMustNotBeZero")
                .field("set_by", set_by)
                .finish(),
            Self::FailedToParseMaxAttempts { source, set_by } => f
                .debug_struct("FailedToParseMaxAttempts")
                .field("source", source)
                .field("set_by", set_by)
                .finish(),
            Self::AdaptiveModeIsNotSupported { set_by } => f
                .debug_struct("AdaptiveModeIsNotSupported")
                .field("set_by", set_by)
                .finish(),
            _ => "???".fmt(f),
        }
    }
}
impl crate::Debug for aws_smithy_types::retry::RetryKind {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Error(__0) => {
                f.debug_tuple("Error").field(__0).finish();
            }
            Self::Explicit(__0) => {
                f.debug_tuple("Explicit").field(__0).finish();
            }
            Self::UnretryableFailure => {
                f.debug_tuple("UnretryableFailure").finish();
            }
            Self::Unnecessary => {
                f.debug_tuple("Unnecessary").finish();
            }
            _ => "???".fmt(f),
        }
    }
}
impl crate::Debug for aws_smithy_types::retry::RetryMode {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Standard => {
                f.debug_tuple("Standard").finish();
            }
            Self::Adaptive => {
                f.debug_tuple("Adaptive").finish();
            }
            _ => "???".fmt(f),
        }
    }
}
// Skipping aws_smithy_types::retry::RetryModeParseErr due to hidden fields
// Skipping aws_smithy_types::timeout::Api due to hidden fields
impl crate::Debug for aws_smithy_types::timeout::Config {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Config")
            .field("api", &self.api)
            .field("http", &self.http)
            .field("tcp", &self.tcp)
            .finish()
    }
}
impl crate::Debug for aws_smithy_types::timeout::ConfigError {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::InvalidTimeout {
                name,
                reason,
                set_by,
            } => f
                .debug_struct("InvalidTimeout")
                .field("name", name)
                .field("reason", reason)
                .field("set_by", set_by)
                .finish(),
            Self::ParseError {
                name,
                set_by,
                source,
            } => f
                .debug_struct("ParseError")
                .field("name", name)
                .field("set_by", set_by)
                .field("source", source)
                .finish(),
            _ => "???".fmt(f),
        }
    }
}
// Skipping aws_smithy_types::timeout::Http due to hidden fields
// Skipping aws_smithy_types::timeout::Tcp due to hidden fields
impl<T> crate::Debug for aws_smithy_types::tristate::TriState<T>
where
    T: crate::Debug,
{
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Unset => {
                f.debug_tuple("Unset").finish();
            }
            Self::Disabled => {
                f.debug_tuple("Disabled").finish();
            }
            Self::Set(__0) => {
                f.debug_tuple("Set").field(__0).finish();
            }
        }
    }
}
