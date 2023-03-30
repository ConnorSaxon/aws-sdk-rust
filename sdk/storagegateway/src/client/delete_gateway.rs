// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteGateway`](crate::client::fluent_builders::DeleteGateway) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteGateway::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::client::fluent_builders::DeleteGateway::set_gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
                            /// - On success, responds with [`DeleteGatewayOutput`](crate::output::DeleteGatewayOutput) with field(s):
    ///   - [`gateway_arn(Option<String>)`](crate::output::DeleteGatewayOutput::gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
                            /// - On failure, responds with [`SdkError<DeleteGatewayError>`](crate::error::DeleteGatewayError)
    pub fn delete_gateway(&self) -> crate::client::fluent_builders::DeleteGateway {
                                crate::client::fluent_builders::DeleteGateway::new(self.handle.clone())
                            }
}

