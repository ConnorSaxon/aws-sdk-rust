// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `InvokeEndpoint`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`invoke_endpoint`](crate::client::fluent_builders::InvokeEndpoint).
            ///
            /// `ParseStrictResponse` impl for `InvokeEndpoint`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct InvokeEndpoint {
    _private: ()
}
impl InvokeEndpoint {
    /// Creates a new builder-style object to manufacture [`InvokeEndpointInput`](crate::input::InvokeEndpointInput).
    pub fn builder() -> crate::input::invoke_endpoint_input::Builder {
        crate::input::invoke_endpoint_input::Builder::default()
    }
    /// Creates a new `InvokeEndpoint` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for InvokeEndpoint {
                type Output = std::result::Result<crate::output::InvokeEndpointOutput, crate::error::InvokeEndpointError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_invoke_endpoint::de_invoke_endpoint_http_error(response)
                     } else {
                        crate::protocol_serde::shape_invoke_endpoint::de_invoke_endpoint_http_response(response)
                     }
                }
            }

/// Operation shape for `InvokeEndpointAsync`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`invoke_endpoint_async`](crate::client::fluent_builders::InvokeEndpointAsync).
            ///
            /// `ParseStrictResponse` impl for `InvokeEndpointAsync`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct InvokeEndpointAsync {
    _private: ()
}
impl InvokeEndpointAsync {
    /// Creates a new builder-style object to manufacture [`InvokeEndpointAsyncInput`](crate::input::InvokeEndpointAsyncInput).
    pub fn builder() -> crate::input::invoke_endpoint_async_input::Builder {
        crate::input::invoke_endpoint_async_input::Builder::default()
    }
    /// Creates a new `InvokeEndpointAsync` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for InvokeEndpointAsync {
                type Output = std::result::Result<crate::output::InvokeEndpointAsyncOutput, crate::error::InvokeEndpointAsyncError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::protocol_serde::shape_invoke_endpoint_async::de_invoke_endpoint_async_http_error(response)
                     } else {
                        crate::protocol_serde::shape_invoke_endpoint_async::de_invoke_endpoint_async_http_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

