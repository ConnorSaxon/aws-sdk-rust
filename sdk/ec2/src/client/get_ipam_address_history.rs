// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetIpamAddressHistory`](crate::client::fluent_builders::GetIpamAddressHistory) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetIpamAddressHistory::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::GetIpamAddressHistory::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::GetIpamAddressHistory::set_dry_run): <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`cidr(impl Into<String>)`](crate::client::fluent_builders::GetIpamAddressHistory::cidr) / [`set_cidr(Option<String>)`](crate::client::fluent_builders::GetIpamAddressHistory::set_cidr): <p>The CIDR you want the history of. The CIDR can be an IPv4 or IPv6 IP address range. If you enter a /16 IPv4 CIDR, you will get records that match it exactly. You will not get records for any subnets within the /16 CIDR.</p>
    ///   - [`ipam_scope_id(impl Into<String>)`](crate::client::fluent_builders::GetIpamAddressHistory::ipam_scope_id) / [`set_ipam_scope_id(Option<String>)`](crate::client::fluent_builders::GetIpamAddressHistory::set_ipam_scope_id): <p>The ID of the IPAM scope that the CIDR is in.</p>
    ///   - [`vpc_id(impl Into<String>)`](crate::client::fluent_builders::GetIpamAddressHistory::vpc_id) / [`set_vpc_id(Option<String>)`](crate::client::fluent_builders::GetIpamAddressHistory::set_vpc_id): <p>The ID of the VPC you want your history records filtered by.</p>
    ///   - [`start_time(DateTime)`](crate::client::fluent_builders::GetIpamAddressHistory::start_time) / [`set_start_time(Option<DateTime>)`](crate::client::fluent_builders::GetIpamAddressHistory::set_start_time): <p>The start of the time period for which you are looking for history. If you omit this option, it will default to the value of EndTime.</p>
    ///   - [`end_time(DateTime)`](crate::client::fluent_builders::GetIpamAddressHistory::end_time) / [`set_end_time(Option<DateTime>)`](crate::client::fluent_builders::GetIpamAddressHistory::set_end_time): <p>The end of the time period for which you are looking for history. If you omit this option, it will default to the current time.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetIpamAddressHistory::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetIpamAddressHistory::set_max_results): <p>The maximum number of historical results you would like returned per page. Defaults to 100.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetIpamAddressHistory::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetIpamAddressHistory::set_next_token): <p>The token for the next page of results.</p>
                            /// - On success, responds with [`GetIpamAddressHistoryOutput`](crate::output::GetIpamAddressHistoryOutput) with field(s):
    ///   - [`history_records(Option<Vec<IpamAddressHistoryRecord>>)`](crate::output::GetIpamAddressHistoryOutput::history_records): <p>A historical record for a CIDR within an IPAM scope. If the CIDR is associated with an EC2 instance, you will see an object in the response for the instance and one for the network interface.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetIpamAddressHistoryOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
                            /// - On failure, responds with [`SdkError<GetIpamAddressHistoryError>`](crate::error::GetIpamAddressHistoryError)
    pub fn get_ipam_address_history(&self) -> crate::client::fluent_builders::GetIpamAddressHistory {
                                crate::client::fluent_builders::GetIpamAddressHistory::new(self.handle.clone())
                            }
}

