// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Concurrent updates caused an exception, for example, if you request an update to a scaling plan that already has a pending update.</p>
    ConcurrentUpdateException(crate::error::ConcurrentUpdateException),
    /// <p>The service encountered an internal error.</p>
    InternalServiceException(crate::error::InternalServiceException),
    /// <p>The token provided is not valid.</p>
    InvalidNextTokenException(crate::error::InvalidNextTokenException),
    /// <p>Your account exceeded a limit. This exception is thrown when a per-account resource limit is exceeded.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The specified object could not be found.</p>
    ObjectNotFoundException(crate::error::ObjectNotFoundException),
    /// <p>An exception was thrown for a validation issue. Review the parameters provided.</p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConcurrentUpdateException(inner) => inner.fmt(f),
            Error::InternalServiceException(inner) => inner.fmt(f),
            Error::InvalidNextTokenException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ObjectNotFoundException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateScalingPlanError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::CreateScalingPlanError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateScalingPlanErrorKind::ConcurrentUpdateException(inner) => {
                    Error::ConcurrentUpdateException(inner)
                }
                crate::error::CreateScalingPlanErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::CreateScalingPlanErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateScalingPlanErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateScalingPlanErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteScalingPlanError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteScalingPlanError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteScalingPlanErrorKind::ConcurrentUpdateException(inner) => {
                    Error::ConcurrentUpdateException(inner)
                }
                crate::error::DeleteScalingPlanErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::DeleteScalingPlanErrorKind::ObjectNotFoundException(inner) => {
                    Error::ObjectNotFoundException(inner)
                }
                crate::error::DeleteScalingPlanErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteScalingPlanErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeScalingPlanResourcesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeScalingPlanResourcesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeScalingPlanResourcesErrorKind::ConcurrentUpdateException(
                    inner,
                ) => Error::ConcurrentUpdateException(inner),
                crate::error::DescribeScalingPlanResourcesErrorKind::InternalServiceException(
                    inner,
                ) => Error::InternalServiceException(inner),
                crate::error::DescribeScalingPlanResourcesErrorKind::InvalidNextTokenException(
                    inner,
                ) => Error::InvalidNextTokenException(inner),
                crate::error::DescribeScalingPlanResourcesErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeScalingPlanResourcesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeScalingPlansError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribeScalingPlansError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeScalingPlansErrorKind::ConcurrentUpdateException(inner) => {
                    Error::ConcurrentUpdateException(inner)
                }
                crate::error::DescribeScalingPlansErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::DescribeScalingPlansErrorKind::InvalidNextTokenException(inner) => {
                    Error::InvalidNextTokenException(inner)
                }
                crate::error::DescribeScalingPlansErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DescribeScalingPlansErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<crate::error::GetScalingPlanResourceForecastDataError, R>,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetScalingPlanResourceForecastDataError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetScalingPlanResourceForecastDataErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::GetScalingPlanResourceForecastDataErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetScalingPlanResourceForecastDataErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateScalingPlanError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::UpdateScalingPlanError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateScalingPlanErrorKind::ConcurrentUpdateException(inner) => {
                    Error::ConcurrentUpdateException(inner)
                }
                crate::error::UpdateScalingPlanErrorKind::InternalServiceException(inner) => {
                    Error::InternalServiceException(inner)
                }
                crate::error::UpdateScalingPlanErrorKind::ObjectNotFoundException(inner) => {
                    Error::ObjectNotFoundException(inner)
                }
                crate::error::UpdateScalingPlanErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateScalingPlanErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
