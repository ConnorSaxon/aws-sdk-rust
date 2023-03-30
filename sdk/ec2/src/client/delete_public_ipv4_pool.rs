// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeletePublicIpv4Pool`](crate::client::fluent_builders::DeletePublicIpv4Pool) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DeletePublicIpv4Pool::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DeletePublicIpv4Pool::set_dry_run): <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`pool_id(impl Into<String>)`](crate::client::fluent_builders::DeletePublicIpv4Pool::pool_id) / [`set_pool_id(Option<String>)`](crate::client::fluent_builders::DeletePublicIpv4Pool::set_pool_id): <p>The ID of the public IPv4 pool you want to delete.</p>
                            /// - On success, responds with [`DeletePublicIpv4PoolOutput`](crate::output::DeletePublicIpv4PoolOutput) with field(s):
    ///   - [`return_value(Option<bool>)`](crate::output::DeletePublicIpv4PoolOutput::return_value): <p>Information about the result of deleting the public IPv4 pool.</p>
                            /// - On failure, responds with [`SdkError<DeletePublicIpv4PoolError>`](crate::error::DeletePublicIpv4PoolError)
    pub fn delete_public_ipv4_pool(&self) -> crate::client::fluent_builders::DeletePublicIpv4Pool {
                                crate::client::fluent_builders::DeletePublicIpv4Pool::new(self.handle.clone())
                            }
}

