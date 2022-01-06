// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p> Account Action is required in order to continue the request. </p>
    AccountActionRequiredException(crate::error::AccountActionRequiredException),
    /// <p> The request cannot be processed because some parameter is not valid or the project state prevents the operation from being performed. </p>
    BadRequestException(crate::error::BadRequestException),
    /// <p> The service has encountered an unexpected error condition which prevents it from servicing the request. </p>
    InternalFailureException(crate::error::InternalFailureException),
    /// <p> There are too many AWS Mobile Hub projects in the account or the account has exceeded the maximum number of resources in some AWS service. You should create another sub-account using AWS Organizations or remove some resources and retry your request. </p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p> No entity can be found with the specified identifier. </p>
    NotFoundException(crate::error::NotFoundException),
    /// <p> The service is temporarily unavailable. The request should be retried after some time delay. </p>
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    /// <p> Too many requests have been received for this AWS account in too short a time. The request should be retried after some time delay. </p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// <p> Credentials of the caller are insufficient to authorize the request. </p>
    UnauthorizedException(crate::error::UnauthorizedException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccountActionRequiredException(inner) => inner.fmt(f),
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::InternalFailureException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::TooManyRequestsException(inner) => inner.fmt(f),
            Error::UnauthorizedException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateProjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateProjectErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::CreateProjectErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::CreateProjectErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateProjectErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::CreateProjectErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::CreateProjectErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::CreateProjectErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::CreateProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteProjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteProjectErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::DeleteProjectErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DeleteProjectErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::DeleteProjectErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DeleteProjectErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::DeleteProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeBundleError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeBundleError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeBundleErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DescribeBundleErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::DescribeBundleErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DescribeBundleErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::DescribeBundleErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DescribeBundleErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::DescribeBundleErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeProjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DescribeProjectErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::DescribeProjectErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::DescribeProjectErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::DescribeProjectErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::DescribeProjectErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::DescribeProjectErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::DescribeProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ExportBundleError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ExportBundleError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ExportBundleErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::ExportBundleErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::ExportBundleErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::ExportBundleErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::ExportBundleErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ExportBundleErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::ExportBundleErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ExportProjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ExportProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ExportProjectErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::ExportProjectErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::ExportProjectErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::ExportProjectErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::ExportProjectErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ExportProjectErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::ExportProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListBundlesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListBundlesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListBundlesErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::ListBundlesErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::ListBundlesErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::ListBundlesErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListBundlesErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::ListBundlesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListProjectsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListProjectsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListProjectsErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::ListProjectsErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::ListProjectsErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::ListProjectsErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::ListProjectsErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::ListProjectsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateProjectError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateProjectErrorKind::AccountActionRequiredException(inner) => {
                    Error::AccountActionRequiredException(inner)
                }
                crate::error::UpdateProjectErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::UpdateProjectErrorKind::InternalFailureException(inner) => {
                    Error::InternalFailureException(inner)
                }
                crate::error::UpdateProjectErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::UpdateProjectErrorKind::NotFoundException(inner) => {
                    Error::NotFoundException(inner)
                }
                crate::error::UpdateProjectErrorKind::ServiceUnavailableException(inner) => {
                    Error::ServiceUnavailableException(inner)
                }
                crate::error::UpdateProjectErrorKind::TooManyRequestsException(inner) => {
                    Error::TooManyRequestsException(inner)
                }
                crate::error::UpdateProjectErrorKind::UnauthorizedException(inner) => {
                    Error::UnauthorizedException(inner)
                }
                crate::error::UpdateProjectErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
