// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// An exception thrown when a bulk publish operation is requested less than 24 hours after a previous bulk publish operation completed successfully.
    AlreadyStreamedException(crate::error::AlreadyStreamedException),
    /// <p>Thrown if there are parallel requests to modify a resource.</p>
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    /// An exception thrown when there is an IN_PROGRESS bulk publish operation for the given identity pool.
    DuplicateRequestException(crate::error::DuplicateRequestException),
    /// Indicates an internal service error.
    InternalErrorException(crate::error::InternalErrorException),
    #[allow(missing_docs)] // documentation missing in model
    InvalidConfigurationException(crate::error::InvalidConfigurationException),
    /// <p>The AWS Lambda function returned invalid output or an exception.</p>
    InvalidLambdaFunctionOutputException(crate::error::InvalidLambdaFunctionOutputException),
    /// Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameterException(crate::error::InvalidParameterException),
    /// <p>AWS Lambda throttled your account, please contact AWS Support</p>
    LambdaThrottledException(crate::error::LambdaThrottledException),
    /// Thrown when the limit on the number of objects or operations has been exceeded.
    LimitExceededException(crate::error::LimitExceededException),
    /// Thrown when a user is not authorized to access the requested resource.
    NotAuthorizedException(crate::error::NotAuthorizedException),
    /// Thrown if an update can't be applied because the resource was changed by another call and this would result in a conflict.
    ResourceConflictException(crate::error::ResourceConflictException),
    /// Thrown if the resource doesn't exist.
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// Thrown if the request is throttled.
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AlreadyStreamedException(inner) => inner.fmt(f),
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::DuplicateRequestException(inner) => inner.fmt(f),
            Error::InternalErrorException(inner) => inner.fmt(f),
            Error::InvalidConfigurationException(inner) => inner.fmt(f),
            Error::InvalidLambdaFunctionOutputException(inner) => inner.fmt(f),
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::LambdaThrottledException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::NotAuthorizedException(inner) => inner.fmt(f),
            Error::ResourceConflictException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::TooManyRequestsException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BulkPublishError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::BulkPublishError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::BulkPublishErrorKind::AlreadyStreamedException(inner) => {
                    Error::AlreadyStreamedException(inner)
                }
                crate::error::BulkPublishErrorKind::DuplicateRequestException(inner) => {
                    Error::DuplicateRequestException(inner)
                }
                crate::error::BulkPublishErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::BulkPublishErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::BulkPublishErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::BulkPublishErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::BulkPublishErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteDatasetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteDatasetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteDatasetErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::DeleteDatasetErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DeleteDatasetErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::DeleteDatasetErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::DeleteDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteDatasetErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DeleteDatasetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeDatasetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeDatasetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeDatasetErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::DescribeDatasetErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeDatasetErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::DescribeDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeDatasetErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DescribeDatasetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeIdentityPoolUsageError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeIdentityPoolUsageError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeIdentityPoolUsageErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::DescribeIdentityPoolUsageErrorKind::InvalidParameterException(
                    inner,
                ) => Error::InvalidParameterException(inner),
                crate::error::DescribeIdentityPoolUsageErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::DescribeIdentityPoolUsageErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::DescribeIdentityPoolUsageErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::DescribeIdentityPoolUsageErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeIdentityUsageError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeIdentityUsageError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeIdentityUsageErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::DescribeIdentityUsageErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::DescribeIdentityUsageErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::DescribeIdentityUsageErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DescribeIdentityUsageErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DescribeIdentityUsageErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetBulkPublishDetailsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetBulkPublishDetailsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetBulkPublishDetailsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::GetBulkPublishDetailsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::GetBulkPublishDetailsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::GetBulkPublishDetailsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetBulkPublishDetailsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetCognitoEventsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetCognitoEventsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetCognitoEventsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::GetCognitoEventsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::GetCognitoEventsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::GetCognitoEventsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetCognitoEventsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::GetCognitoEventsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetIdentityPoolConfigurationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetIdentityPoolConfigurationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetIdentityPoolConfigurationErrorKind::InternalErrorException(
                    inner,
                ) => Error::InternalErrorException(inner),
                crate::error::GetIdentityPoolConfigurationErrorKind::InvalidParameterException(
                    inner,
                ) => Error::InvalidParameterException(inner),
                crate::error::GetIdentityPoolConfigurationErrorKind::NotAuthorizedException(
                    inner,
                ) => Error::NotAuthorizedException(inner),
                crate::error::GetIdentityPoolConfigurationErrorKind::ResourceNotFoundException(
                    inner,
                ) => Error::ResourceNotFoundException(inner),
                crate::error::GetIdentityPoolConfigurationErrorKind::TooManyRequestsException(
                    inner,
                ) => Error::TooManyRequestsException(inner),
                crate::error::GetIdentityPoolConfigurationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListDatasetsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListDatasetsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListDatasetsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::ListDatasetsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::ListDatasetsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::ListDatasetsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListDatasetsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListIdentityPoolUsageError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListIdentityPoolUsageError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListIdentityPoolUsageErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::ListIdentityPoolUsageErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::ListIdentityPoolUsageErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::ListIdentityPoolUsageErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListIdentityPoolUsageErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListRecordsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListRecordsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListRecordsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::ListRecordsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::ListRecordsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::ListRecordsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListRecordsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RegisterDeviceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::RegisterDeviceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RegisterDeviceErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::RegisterDeviceErrorKind::InvalidConfigurationException(inner) => {
                    Error::InvalidConfigurationException(inner)
                }
                crate::error::RegisterDeviceErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::RegisterDeviceErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::RegisterDeviceErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::RegisterDeviceErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::RegisterDeviceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SetCognitoEventsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::SetCognitoEventsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SetCognitoEventsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::SetCognitoEventsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::SetCognitoEventsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::SetCognitoEventsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::SetCognitoEventsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::SetCognitoEventsErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SetIdentityPoolConfigurationError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::SetIdentityPoolConfigurationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::SetIdentityPoolConfigurationErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
                crate::error::SetIdentityPoolConfigurationErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
                crate::error::SetIdentityPoolConfigurationErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::SetIdentityPoolConfigurationErrorKind::NotAuthorizedException(inner) => Error::NotAuthorizedException(inner),
                crate::error::SetIdentityPoolConfigurationErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::SetIdentityPoolConfigurationErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
                crate::error::SetIdentityPoolConfigurationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SubscribeToDatasetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::SubscribeToDatasetError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SubscribeToDatasetErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::SubscribeToDatasetErrorKind::InvalidConfigurationException(inner) => {
                    Error::InvalidConfigurationException(inner)
                }
                crate::error::SubscribeToDatasetErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::SubscribeToDatasetErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::SubscribeToDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::SubscribeToDatasetErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::SubscribeToDatasetErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UnsubscribeFromDatasetError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UnsubscribeFromDatasetError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UnsubscribeFromDatasetErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::UnsubscribeFromDatasetErrorKind::InvalidConfigurationException(
                    inner,
                ) => Error::InvalidConfigurationException(inner),
                crate::error::UnsubscribeFromDatasetErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::UnsubscribeFromDatasetErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::UnsubscribeFromDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UnsubscribeFromDatasetErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::UnsubscribeFromDatasetErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateRecordsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateRecordsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateRecordsErrorKind::InternalErrorException(inner) => {
                    Error::InternalErrorException(inner)
                }
                crate::error::UpdateRecordsErrorKind::InvalidLambdaFunctionOutputException(
                    inner,
                ) => Error::InvalidLambdaFunctionOutputException(inner),
                crate::error::UpdateRecordsErrorKind::InvalidParameterException(inner) => {
                    Error::InvalidParameterException(inner)
                }
                crate::error::UpdateRecordsErrorKind::LambdaThrottledException(inner) => {
                    Error::LambdaThrottledException(inner)
                }
                crate::error::UpdateRecordsErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::UpdateRecordsErrorKind::NotAuthorizedException(inner) => {
                    Error::NotAuthorizedException(inner)
                }
                crate::error::UpdateRecordsErrorKind::ResourceConflictException(inner) => {
                    Error::ResourceConflictException(inner)
                }
                crate::error::UpdateRecordsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateRecordsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::UpdateRecordsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
