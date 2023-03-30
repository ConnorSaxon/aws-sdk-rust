// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DetachInternetGateway`](crate::client::fluent_builders::DetachInternetGateway) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DetachInternetGateway::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DetachInternetGateway::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`internet_gateway_id(impl Into<String>)`](crate::client::fluent_builders::DetachInternetGateway::internet_gateway_id) / [`set_internet_gateway_id(Option<String>)`](crate::client::fluent_builders::DetachInternetGateway::set_internet_gateway_id): <p>The ID of the internet gateway.</p>
    ///   - [`vpc_id(impl Into<String>)`](crate::client::fluent_builders::DetachInternetGateway::vpc_id) / [`set_vpc_id(Option<String>)`](crate::client::fluent_builders::DetachInternetGateway::set_vpc_id): <p>The ID of the VPC.</p>
                            /// - On success, responds with [`DetachInternetGatewayOutput`](crate::output::DetachInternetGatewayOutput)
                            /// - On failure, responds with [`SdkError<DetachInternetGatewayError>`](crate::error::DetachInternetGatewayError)
    pub fn detach_internet_gateway(&self) -> crate::client::fluent_builders::DetachInternetGateway {
                                crate::client::fluent_builders::DetachInternetGateway::new(self.handle.clone())
                            }
}

