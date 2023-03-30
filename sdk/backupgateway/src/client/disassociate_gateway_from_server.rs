// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DisassociateGatewayFromServer`](crate::client::fluent_builders::DisassociateGatewayFromServer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl Into<String>)`](crate::client::fluent_builders::DisassociateGatewayFromServer::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::client::fluent_builders::DisassociateGatewayFromServer::set_gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway to disassociate.</p>
                            /// - On success, responds with [`DisassociateGatewayFromServerOutput`](crate::output::DisassociateGatewayFromServerOutput) with field(s):
    ///   - [`gateway_arn(Option<String>)`](crate::output::DisassociateGatewayFromServerOutput::gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway you disassociated.</p>
                            /// - On failure, responds with [`SdkError<DisassociateGatewayFromServerError>`](crate::error::DisassociateGatewayFromServerError)
    pub fn disassociate_gateway_from_server(&self) -> crate::client::fluent_builders::DisassociateGatewayFromServer {
                                crate::client::fluent_builders::DisassociateGatewayFromServer::new(self.handle.clone())
                            }
}

