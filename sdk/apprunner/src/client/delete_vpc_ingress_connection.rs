// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteVpcIngressConnection`](crate::client::fluent_builders::DeleteVpcIngressConnection) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`vpc_ingress_connection_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteVpcIngressConnection::vpc_ingress_connection_arn) / [`set_vpc_ingress_connection_arn(Option<String>)`](crate::client::fluent_builders::DeleteVpcIngressConnection::set_vpc_ingress_connection_arn): <p>The Amazon Resource Name (ARN) of the App Runner VPC Ingress Connection that you want to delete.</p>
                            /// - On success, responds with [`DeleteVpcIngressConnectionOutput`](crate::output::DeleteVpcIngressConnectionOutput) with field(s):
    ///   - [`vpc_ingress_connection(Option<VpcIngressConnection>)`](crate::output::DeleteVpcIngressConnectionOutput::vpc_ingress_connection): <p>A description of the App Runner VPC Ingress Connection that this request just deleted.</p>
                            /// - On failure, responds with [`SdkError<DeleteVpcIngressConnectionError>`](crate::error::DeleteVpcIngressConnectionError)
    pub fn delete_vpc_ingress_connection(&self) -> crate::client::fluent_builders::DeleteVpcIngressConnection {
                                crate::client::fluent_builders::DeleteVpcIngressConnection::new(self.handle.clone())
                            }
}

