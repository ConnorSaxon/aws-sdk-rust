// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DeleteHumanLoop`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_human_loop`](crate::client::fluent_builders::DeleteHumanLoop).
            ///
            /// `ParseStrictResponse` impl for `DeleteHumanLoop`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteHumanLoop {
    _private: ()
}
impl DeleteHumanLoop {
    /// Creates a new builder-style object to manufacture [`DeleteHumanLoopInput`](crate::input::DeleteHumanLoopInput).
    pub fn builder() -> crate::input::delete_human_loop_input::Builder {
        crate::input::delete_human_loop_input::Builder::default()
    }
    /// Creates a new `DeleteHumanLoop` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteHumanLoop {
                type Output = std::result::Result<crate::output::DeleteHumanLoopOutput, crate::error::DeleteHumanLoopError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_delete_human_loop::de_delete_human_loop_http_error(response)
                     } else {
                        crate::protocol_serde::shape_delete_human_loop::de_delete_human_loop_http_response(response)
                     }
                }
            }

/// Operation shape for `DescribeHumanLoop`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_human_loop`](crate::client::fluent_builders::DescribeHumanLoop).
            ///
            /// `ParseStrictResponse` impl for `DescribeHumanLoop`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeHumanLoop {
    _private: ()
}
impl DescribeHumanLoop {
    /// Creates a new builder-style object to manufacture [`DescribeHumanLoopInput`](crate::input::DescribeHumanLoopInput).
    pub fn builder() -> crate::input::describe_human_loop_input::Builder {
        crate::input::describe_human_loop_input::Builder::default()
    }
    /// Creates a new `DescribeHumanLoop` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeHumanLoop {
                type Output = std::result::Result<crate::output::DescribeHumanLoopOutput, crate::error::DescribeHumanLoopError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_describe_human_loop::de_describe_human_loop_http_error(response)
                     } else {
                        crate::protocol_serde::shape_describe_human_loop::de_describe_human_loop_http_response(response)
                     }
                }
            }

/// Operation shape for `ListHumanLoops`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_human_loops`](crate::client::fluent_builders::ListHumanLoops).
            ///
            /// `ParseStrictResponse` impl for `ListHumanLoops`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListHumanLoops {
    _private: ()
}
impl ListHumanLoops {
    /// Creates a new builder-style object to manufacture [`ListHumanLoopsInput`](crate::input::ListHumanLoopsInput).
    pub fn builder() -> crate::input::list_human_loops_input::Builder {
        crate::input::list_human_loops_input::Builder::default()
    }
    /// Creates a new `ListHumanLoops` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListHumanLoops {
                type Output = std::result::Result<crate::output::ListHumanLoopsOutput, crate::error::ListHumanLoopsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_human_loops::de_list_human_loops_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_human_loops::de_list_human_loops_http_response(response)
                     }
                }
            }

/// Operation shape for `StartHumanLoop`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_human_loop`](crate::client::fluent_builders::StartHumanLoop).
            ///
            /// `ParseStrictResponse` impl for `StartHumanLoop`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartHumanLoop {
    _private: ()
}
impl StartHumanLoop {
    /// Creates a new builder-style object to manufacture [`StartHumanLoopInput`](crate::input::StartHumanLoopInput).
    pub fn builder() -> crate::input::start_human_loop_input::Builder {
        crate::input::start_human_loop_input::Builder::default()
    }
    /// Creates a new `StartHumanLoop` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartHumanLoop {
                type Output = std::result::Result<crate::output::StartHumanLoopOutput, crate::error::StartHumanLoopError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_start_human_loop::de_start_human_loop_http_error(response)
                     } else {
                        crate::protocol_serde::shape_start_human_loop::de_start_human_loop_http_response(response)
                     }
                }
            }

/// Operation shape for `StopHumanLoop`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`stop_human_loop`](crate::client::fluent_builders::StopHumanLoop).
            ///
            /// `ParseStrictResponse` impl for `StopHumanLoop`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StopHumanLoop {
    _private: ()
}
impl StopHumanLoop {
    /// Creates a new builder-style object to manufacture [`StopHumanLoopInput`](crate::input::StopHumanLoopInput).
    pub fn builder() -> crate::input::stop_human_loop_input::Builder {
        crate::input::stop_human_loop_input::Builder::default()
    }
    /// Creates a new `StopHumanLoop` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopHumanLoop {
                type Output = std::result::Result<crate::output::StopHumanLoopOutput, crate::error::StopHumanLoopError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_stop_human_loop::de_stop_human_loop_http_error(response)
                     } else {
                        crate::protocol_serde::shape_stop_human_loop::de_stop_human_loop_http_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

