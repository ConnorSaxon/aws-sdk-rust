// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `DeleteAlternateContact` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DeleteAlternateContactError {
    /// Kind of error that occurred.
    pub kind: DeleteAlternateContactErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `DeleteAlternateContact` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DeleteAlternateContactErrorKind {
    /// <p>The operation failed because the calling identity doesn't have the minimum required permissions.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>The operation failed because of an error internal to Amazon Web Services. Try your operation again later.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The operation failed because it specified a resource that can't be found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The operation failed because it was called too frequently and exceeded a throttle limit.</p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// <p>The operation failed because one of the input parameters was invalid.</p>
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DeleteAlternateContactError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DeleteAlternateContactErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            DeleteAlternateContactErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            DeleteAlternateContactErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            DeleteAlternateContactErrorKind::TooManyRequestsException(_inner) => _inner.fmt(f),
            DeleteAlternateContactErrorKind::ValidationException(_inner) => _inner.fmt(f),
            DeleteAlternateContactErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DeleteAlternateContactError {
    fn code(&self) -> Option<&str> {
        DeleteAlternateContactError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        match &self.kind {
            DeleteAlternateContactErrorKind::InternalServerException(inner) => {
                Some(inner.retryable_error_kind())
            }
            DeleteAlternateContactErrorKind::TooManyRequestsException(inner) => {
                Some(inner.retryable_error_kind())
            }
            _ => None,
        }
    }
}
impl DeleteAlternateContactError {
    /// Creates a new `DeleteAlternateContactError`.
    pub fn new(kind: DeleteAlternateContactErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `DeleteAlternateContactError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DeleteAlternateContactErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `DeleteAlternateContactError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DeleteAlternateContactErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `DeleteAlternateContactErrorKind::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteAlternateContactErrorKind::AccessDeniedException(_)
        )
    }
    /// Returns `true` if the error kind is `DeleteAlternateContactErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteAlternateContactErrorKind::InternalServerException(_)
        )
    }
    /// Returns `true` if the error kind is `DeleteAlternateContactErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteAlternateContactErrorKind::ResourceNotFoundException(_)
        )
    }
    /// Returns `true` if the error kind is `DeleteAlternateContactErrorKind::TooManyRequestsException`.
    pub fn is_too_many_requests_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteAlternateContactErrorKind::TooManyRequestsException(_)
        )
    }
    /// Returns `true` if the error kind is `DeleteAlternateContactErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteAlternateContactErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for DeleteAlternateContactError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DeleteAlternateContactErrorKind::AccessDeniedException(_inner) => Some(_inner),
            DeleteAlternateContactErrorKind::InternalServerException(_inner) => Some(_inner),
            DeleteAlternateContactErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            DeleteAlternateContactErrorKind::TooManyRequestsException(_inner) => Some(_inner),
            DeleteAlternateContactErrorKind::ValidationException(_inner) => Some(_inner),
            DeleteAlternateContactErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `GetAlternateContact` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetAlternateContactError {
    /// Kind of error that occurred.
    pub kind: GetAlternateContactErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `GetAlternateContact` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetAlternateContactErrorKind {
    /// <p>The operation failed because the calling identity doesn't have the minimum required permissions.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>The operation failed because of an error internal to Amazon Web Services. Try your operation again later.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The operation failed because it specified a resource that can't be found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The operation failed because it was called too frequently and exceeded a throttle limit.</p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// <p>The operation failed because one of the input parameters was invalid.</p>
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GetAlternateContactError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetAlternateContactErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            GetAlternateContactErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            GetAlternateContactErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            GetAlternateContactErrorKind::TooManyRequestsException(_inner) => _inner.fmt(f),
            GetAlternateContactErrorKind::ValidationException(_inner) => _inner.fmt(f),
            GetAlternateContactErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetAlternateContactError {
    fn code(&self) -> Option<&str> {
        GetAlternateContactError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        match &self.kind {
            GetAlternateContactErrorKind::InternalServerException(inner) => {
                Some(inner.retryable_error_kind())
            }
            GetAlternateContactErrorKind::TooManyRequestsException(inner) => {
                Some(inner.retryable_error_kind())
            }
            _ => None,
        }
    }
}
impl GetAlternateContactError {
    /// Creates a new `GetAlternateContactError`.
    pub fn new(kind: GetAlternateContactErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetAlternateContactError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetAlternateContactErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `GetAlternateContactError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetAlternateContactErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `GetAlternateContactErrorKind::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetAlternateContactErrorKind::AccessDeniedException(_)
        )
    }
    /// Returns `true` if the error kind is `GetAlternateContactErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetAlternateContactErrorKind::InternalServerException(_)
        )
    }
    /// Returns `true` if the error kind is `GetAlternateContactErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetAlternateContactErrorKind::ResourceNotFoundException(_)
        )
    }
    /// Returns `true` if the error kind is `GetAlternateContactErrorKind::TooManyRequestsException`.
    pub fn is_too_many_requests_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetAlternateContactErrorKind::TooManyRequestsException(_)
        )
    }
    /// Returns `true` if the error kind is `GetAlternateContactErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetAlternateContactErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for GetAlternateContactError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetAlternateContactErrorKind::AccessDeniedException(_inner) => Some(_inner),
            GetAlternateContactErrorKind::InternalServerException(_inner) => Some(_inner),
            GetAlternateContactErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            GetAlternateContactErrorKind::TooManyRequestsException(_inner) => Some(_inner),
            GetAlternateContactErrorKind::ValidationException(_inner) => Some(_inner),
            GetAlternateContactErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `PutAlternateContact` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct PutAlternateContactError {
    /// Kind of error that occurred.
    pub kind: PutAlternateContactErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `PutAlternateContact` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PutAlternateContactErrorKind {
    /// <p>The operation failed because the calling identity doesn't have the minimum required permissions.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>The operation failed because of an error internal to Amazon Web Services. Try your operation again later.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The operation failed because it was called too frequently and exceeded a throttle limit.</p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// <p>The operation failed because one of the input parameters was invalid.</p>
    ValidationException(crate::error::ValidationException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for PutAlternateContactError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PutAlternateContactErrorKind::AccessDeniedException(_inner) => _inner.fmt(f),
            PutAlternateContactErrorKind::InternalServerException(_inner) => _inner.fmt(f),
            PutAlternateContactErrorKind::TooManyRequestsException(_inner) => _inner.fmt(f),
            PutAlternateContactErrorKind::ValidationException(_inner) => _inner.fmt(f),
            PutAlternateContactErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for PutAlternateContactError {
    fn code(&self) -> Option<&str> {
        PutAlternateContactError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        match &self.kind {
            PutAlternateContactErrorKind::InternalServerException(inner) => {
                Some(inner.retryable_error_kind())
            }
            PutAlternateContactErrorKind::TooManyRequestsException(inner) => {
                Some(inner.retryable_error_kind())
            }
            _ => None,
        }
    }
}
impl PutAlternateContactError {
    /// Creates a new `PutAlternateContactError`.
    pub fn new(kind: PutAlternateContactErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `PutAlternateContactError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: PutAlternateContactErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `PutAlternateContactError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: PutAlternateContactErrorKind::Unhandled(err.into()),
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
    /// Returns `true` if the error kind is `PutAlternateContactErrorKind::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutAlternateContactErrorKind::AccessDeniedException(_)
        )
    }
    /// Returns `true` if the error kind is `PutAlternateContactErrorKind::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutAlternateContactErrorKind::InternalServerException(_)
        )
    }
    /// Returns `true` if the error kind is `PutAlternateContactErrorKind::TooManyRequestsException`.
    pub fn is_too_many_requests_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutAlternateContactErrorKind::TooManyRequestsException(_)
        )
    }
    /// Returns `true` if the error kind is `PutAlternateContactErrorKind::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutAlternateContactErrorKind::ValidationException(_)
        )
    }
}
impl std::error::Error for PutAlternateContactError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            PutAlternateContactErrorKind::AccessDeniedException(_inner) => Some(_inner),
            PutAlternateContactErrorKind::InternalServerException(_inner) => Some(_inner),
            PutAlternateContactErrorKind::TooManyRequestsException(_inner) => Some(_inner),
            PutAlternateContactErrorKind::ValidationException(_inner) => Some(_inner),
            PutAlternateContactErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>The operation failed because one of the input parameters was invalid.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ValidationException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ValidationException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ValidationException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidationException")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for ValidationException {}
/// See [`ValidationException`](crate::error::ValidationException)
pub mod validation_exception {
    /// A builder for [`ValidationException`](crate::error::ValidationException)
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
        /// Consumes the builder and constructs a [`ValidationException`](crate::error::ValidationException)
        pub fn build(self) -> crate::error::ValidationException {
            crate::error::ValidationException {
                message: self.message,
            }
        }
    }
}
impl ValidationException {
    /// Creates a new builder-style object to manufacture [`ValidationException`](crate::error::ValidationException)
    pub fn builder() -> crate::error::validation_exception::Builder {
        crate::error::validation_exception::Builder::default()
    }
}

/// <p>The operation failed because it was called too frequently and exceeded a throttle limit.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct TooManyRequestsException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for TooManyRequestsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TooManyRequestsException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl TooManyRequestsException {
    /// Returns `Some(ErrorKind)` if the error is retryable. Otherwise, returns `None`.
    pub fn retryable_error_kind(&self) -> aws_smithy_types::retry::ErrorKind {
        aws_smithy_types::retry::ErrorKind::ThrottlingError
    }
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for TooManyRequestsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TooManyRequestsException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for TooManyRequestsException {}
/// See [`TooManyRequestsException`](crate::error::TooManyRequestsException)
pub mod too_many_requests_exception {
    /// A builder for [`TooManyRequestsException`](crate::error::TooManyRequestsException)
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
        /// Consumes the builder and constructs a [`TooManyRequestsException`](crate::error::TooManyRequestsException)
        pub fn build(self) -> crate::error::TooManyRequestsException {
            crate::error::TooManyRequestsException {
                message: self.message,
            }
        }
    }
}
impl TooManyRequestsException {
    /// Creates a new builder-style object to manufacture [`TooManyRequestsException`](crate::error::TooManyRequestsException)
    pub fn builder() -> crate::error::too_many_requests_exception::Builder {
        crate::error::too_many_requests_exception::Builder::default()
    }
}

/// <p>The operation failed because of an error internal to Amazon Web Services. Try your operation again later.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalServerException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalServerException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalServerException {
    /// Returns `Some(ErrorKind)` if the error is retryable. Otherwise, returns `None`.
    pub fn retryable_error_kind(&self) -> aws_smithy_types::retry::ErrorKind {
        aws_smithy_types::retry::ErrorKind::ServerError
    }
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServerException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalServerException {}
/// See [`InternalServerException`](crate::error::InternalServerException)
pub mod internal_server_exception {
    /// A builder for [`InternalServerException`](crate::error::InternalServerException)
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
        /// Consumes the builder and constructs a [`InternalServerException`](crate::error::InternalServerException)
        pub fn build(self) -> crate::error::InternalServerException {
            crate::error::InternalServerException {
                message: self.message,
            }
        }
    }
}
impl InternalServerException {
    /// Creates a new builder-style object to manufacture [`InternalServerException`](crate::error::InternalServerException)
    pub fn builder() -> crate::error::internal_server_exception::Builder {
        crate::error::internal_server_exception::Builder::default()
    }
}

/// <p>The operation failed because the calling identity doesn't have the minimum required permissions.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AccessDeniedException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for AccessDeniedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AccessDeniedException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl AccessDeniedException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for AccessDeniedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccessDeniedException")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for AccessDeniedException {}
/// See [`AccessDeniedException`](crate::error::AccessDeniedException)
pub mod access_denied_exception {
    /// A builder for [`AccessDeniedException`](crate::error::AccessDeniedException)
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
        /// Consumes the builder and constructs a [`AccessDeniedException`](crate::error::AccessDeniedException)
        pub fn build(self) -> crate::error::AccessDeniedException {
            crate::error::AccessDeniedException {
                message: self.message,
            }
        }
    }
}
impl AccessDeniedException {
    /// Creates a new builder-style object to manufacture [`AccessDeniedException`](crate::error::AccessDeniedException)
    pub fn builder() -> crate::error::access_denied_exception::Builder {
        crate::error::access_denied_exception::Builder::default()
    }
}

/// <p>The operation failed because it specified a resource that can't be found.</p>
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
        if let Some(inner_5) = &self.message {
            write!(f, ": {}", inner_5)?;
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
