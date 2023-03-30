// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeprovisionPublicIpv4PoolCidr`](crate::client::fluent_builders::DeprovisionPublicIpv4PoolCidr) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::DeprovisionPublicIpv4PoolCidr::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::DeprovisionPublicIpv4PoolCidr::set_dry_run): <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`pool_id(impl Into<String>)`](crate::client::fluent_builders::DeprovisionPublicIpv4PoolCidr::pool_id) / [`set_pool_id(Option<String>)`](crate::client::fluent_builders::DeprovisionPublicIpv4PoolCidr::set_pool_id): <p>The ID of the pool that you want to deprovision the CIDR from.</p>
    ///   - [`cidr(impl Into<String>)`](crate::client::fluent_builders::DeprovisionPublicIpv4PoolCidr::cidr) / [`set_cidr(Option<String>)`](crate::client::fluent_builders::DeprovisionPublicIpv4PoolCidr::set_cidr): <p>The CIDR you want to deprovision from the pool.</p>
                            /// - On success, responds with [`DeprovisionPublicIpv4PoolCidrOutput`](crate::output::DeprovisionPublicIpv4PoolCidrOutput) with field(s):
    ///   - [`pool_id(Option<String>)`](crate::output::DeprovisionPublicIpv4PoolCidrOutput::pool_id): <p>The ID of the pool that you deprovisioned the CIDR from.</p>
    ///   - [`deprovisioned_addresses(Option<Vec<String>>)`](crate::output::DeprovisionPublicIpv4PoolCidrOutput::deprovisioned_addresses): <p>The deprovisioned CIDRs.</p>
                            /// - On failure, responds with [`SdkError<DeprovisionPublicIpv4PoolCidrError>`](crate::error::DeprovisionPublicIpv4PoolCidrError)
    pub fn deprovision_public_ipv4_pool_cidr(&self) -> crate::client::fluent_builders::DeprovisionPublicIpv4PoolCidr {
                                crate::client::fluent_builders::DeprovisionPublicIpv4PoolCidr::new(self.handle.clone())
                            }
}

