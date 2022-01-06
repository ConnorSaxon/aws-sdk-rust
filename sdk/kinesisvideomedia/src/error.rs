// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `GetMedia` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetMediaError {
    /// Kind of error that occurred.
    pub kind: GetMediaErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `GetMedia` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetMediaErrorKind {
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
    ClientLimitExceededException(crate::error::ClientLimitExceededException),
    /// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client connections.</p>
    ConnectionLimitExceededException(crate::error::ConnectionLimitExceededException),
    /// <p>The value for this input parameter is invalid.</p>
    InvalidArgumentException(crate::error::InvalidArgumentException),
    /// <p> Status Code: 400, Caller used wrong endpoint to write data to a stream. On receiving such an exception, the user must call <code>GetDataEndpoint</code> with <code>AccessMode</code> set to "READ" and use the endpoint Kinesis Video returns in the next <code>GetMedia</code> call. </p>
    InvalidEndpointException(crate::error::InvalidEndpointException),
    /// <p>Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired.</p>
    NotAuthorizedException(crate::error::NotAuthorizedException),
    /// <p>Status Code: 404, The stream with the given name does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GetMediaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetMediaErrorKind::ClientLimitExceededException(_inner) => _inner.fmt(f),
            GetMediaErrorKind::ConnectionLimitExceededException(_inner) => _inner.fmt(f),
            GetMediaErrorKind::InvalidArgumentException(_inner) => _inner.fmt(f),
            GetMediaErrorKind::InvalidEndpointException(_inner) => _inner.fmt(f),
            GetMediaErrorKind::NotAuthorizedException(_inner) => _inner.fmt(f),
            GetMediaErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            GetMediaErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetMediaError {
    fn code(&self) -> Option<&str> {
        GetMediaError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetMediaError {
    /// Creates a new `GetMediaError`.
    pub fn new(kind: GetMediaErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetMediaError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetMediaErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `GetMediaError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetMediaErrorKind::Unhandled(err.into()),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `GetMediaErrorKind::ClientLimitExceededException`.
    pub fn is_client_limit_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetMediaErrorKind::ClientLimitExceededException(_)
        )
    }
    /// Returns `true` if the error kind is `GetMediaErrorKind::ConnectionLimitExceededException`.
    pub fn is_connection_limit_exceeded_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetMediaErrorKind::ConnectionLimitExceededException(_)
        )
    }
    /// Returns `true` if the error kind is `GetMediaErrorKind::InvalidArgumentException`.
    pub fn is_invalid_argument_exception(&self) -> bool {
        matches!(&self.kind, GetMediaErrorKind::InvalidArgumentException(_))
    }
    /// Returns `true` if the error kind is `GetMediaErrorKind::InvalidEndpointException`.
    pub fn is_invalid_endpoint_exception(&self) -> bool {
        matches!(&self.kind, GetMediaErrorKind::InvalidEndpointException(_))
    }
    /// Returns `true` if the error kind is `GetMediaErrorKind::NotAuthorizedException`.
    pub fn is_not_authorized_exception(&self) -> bool {
        matches!(&self.kind, GetMediaErrorKind::NotAuthorizedException(_))
    }
    /// Returns `true` if the error kind is `GetMediaErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, GetMediaErrorKind::ResourceNotFoundException(_))
    }
}
impl std::error::Error for GetMediaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetMediaErrorKind::ClientLimitExceededException(_inner) => Some(_inner),
            GetMediaErrorKind::ConnectionLimitExceededException(_inner) => Some(_inner),
            GetMediaErrorKind::InvalidArgumentException(_inner) => Some(_inner),
            GetMediaErrorKind::InvalidEndpointException(_inner) => Some(_inner),
            GetMediaErrorKind::NotAuthorizedException(_inner) => Some(_inner),
            GetMediaErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            GetMediaErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>Status Code: 404, The stream with the given name does not exist.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceNotFoundException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceNotFoundException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ResourceNotFoundException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFoundException {}
/// See [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
pub mod resource_not_found_exception {
    /// A builder for [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message,
            }
        }
    }
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::error::ResourceNotFoundException)
    pub fn builder() -> crate::error::resource_not_found_exception::Builder {
        crate::error::resource_not_found_exception::Builder::default()
    }
}

/// <p>Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct NotAuthorizedException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for NotAuthorizedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("NotAuthorizedException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl NotAuthorizedException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for NotAuthorizedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotAuthorizedException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for NotAuthorizedException {}
/// See [`NotAuthorizedException`](crate::error::NotAuthorizedException)
pub mod not_authorized_exception {
    /// A builder for [`NotAuthorizedException`](crate::error::NotAuthorizedException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`NotAuthorizedException`](crate::error::NotAuthorizedException)
        pub fn build(self) -> crate::error::NotAuthorizedException {
            crate::error::NotAuthorizedException {
                message: self.message,
            }
        }
    }
}
impl NotAuthorizedException {
    /// Creates a new builder-style object to manufacture [`NotAuthorizedException`](crate::error::NotAuthorizedException)
    pub fn builder() -> crate::error::not_authorized_exception::Builder {
        crate::error::not_authorized_exception::Builder::default()
    }
}

/// <p> Status Code: 400, Caller used wrong endpoint to write data to a stream. On receiving such an exception, the user must call <code>GetDataEndpoint</code> with <code>AccessMode</code> set to "READ" and use the endpoint Kinesis Video returns in the next <code>GetMedia</code> call. </p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidEndpointException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidEndpointException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidEndpointException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InvalidEndpointException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidEndpointException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidEndpointException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidEndpointException {}
/// See [`InvalidEndpointException`](crate::error::InvalidEndpointException)
pub mod invalid_endpoint_exception {
    /// A builder for [`InvalidEndpointException`](crate::error::InvalidEndpointException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidEndpointException`](crate::error::InvalidEndpointException)
        pub fn build(self) -> crate::error::InvalidEndpointException {
            crate::error::InvalidEndpointException {
                message: self.message,
            }
        }
    }
}
impl InvalidEndpointException {
    /// Creates a new builder-style object to manufacture [`InvalidEndpointException`](crate::error::InvalidEndpointException)
    pub fn builder() -> crate::error::invalid_endpoint_exception::Builder {
        crate::error::invalid_endpoint_exception::Builder::default()
    }
}

/// <p>The value for this input parameter is invalid.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InvalidArgumentException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InvalidArgumentException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InvalidArgumentException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InvalidArgumentException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidArgumentException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidArgumentException")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for InvalidArgumentException {}
/// See [`InvalidArgumentException`](crate::error::InvalidArgumentException)
pub mod invalid_argument_exception {
    /// A builder for [`InvalidArgumentException`](crate::error::InvalidArgumentException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidArgumentException`](crate::error::InvalidArgumentException)
        pub fn build(self) -> crate::error::InvalidArgumentException {
            crate::error::InvalidArgumentException {
                message: self.message,
            }
        }
    }
}
impl InvalidArgumentException {
    /// Creates a new builder-style object to manufacture [`InvalidArgumentException`](crate::error::InvalidArgumentException)
    pub fn builder() -> crate::error::invalid_argument_exception::Builder {
        crate::error::invalid_argument_exception::Builder::default()
    }
}

/// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client connections.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ConnectionLimitExceededException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ConnectionLimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ConnectionLimitExceededException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ConnectionLimitExceededException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ConnectionLimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConnectionLimitExceededException")?;
        if let Some(inner_5) = &self.message {
            write!(f, ": {}", inner_5)?;
        }
        Ok(())
    }
}
impl std::error::Error for ConnectionLimitExceededException {}
/// See [`ConnectionLimitExceededException`](crate::error::ConnectionLimitExceededException)
pub mod connection_limit_exceeded_exception {
    /// A builder for [`ConnectionLimitExceededException`](crate::error::ConnectionLimitExceededException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ConnectionLimitExceededException`](crate::error::ConnectionLimitExceededException)
        pub fn build(self) -> crate::error::ConnectionLimitExceededException {
            crate::error::ConnectionLimitExceededException {
                message: self.message,
            }
        }
    }
}
impl ConnectionLimitExceededException {
    /// Creates a new builder-style object to manufacture [`ConnectionLimitExceededException`](crate::error::ConnectionLimitExceededException)
    pub fn builder() -> crate::error::connection_limit_exceeded_exception::Builder {
        crate::error::connection_limit_exceeded_exception::Builder::default()
    }
}

/// <p>Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. Try making the call later.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ClientLimitExceededException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ClientLimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ClientLimitExceededException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ClientLimitExceededException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ClientLimitExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientLimitExceededException")?;
        if let Some(inner_6) = &self.message {
            write!(f, ": {}", inner_6)?;
        }
        Ok(())
    }
}
impl std::error::Error for ClientLimitExceededException {}
/// See [`ClientLimitExceededException`](crate::error::ClientLimitExceededException)
pub mod client_limit_exceeded_exception {
    /// A builder for [`ClientLimitExceededException`](crate::error::ClientLimitExceededException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ClientLimitExceededException`](crate::error::ClientLimitExceededException)
        pub fn build(self) -> crate::error::ClientLimitExceededException {
            crate::error::ClientLimitExceededException {
                message: self.message,
            }
        }
    }
}
impl ClientLimitExceededException {
    /// Creates a new builder-style object to manufacture [`ClientLimitExceededException`](crate::error::ClientLimitExceededException)
    pub fn builder() -> crate::error::client_limit_exceeded_exception::Builder {
        crate::error::client_limit_exceeded_exception::Builder::default()
    }
}
