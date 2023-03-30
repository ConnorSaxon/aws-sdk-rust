// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDistributedGrants`](crate::client::fluent_builders::ListDistributedGrants) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`grant_arns(Vec<String>)`](crate::client::fluent_builders::ListDistributedGrants::grant_arns) / [`set_grant_arns(Option<Vec<String>>)`](crate::client::fluent_builders::ListDistributedGrants::set_grant_arns): <p>Amazon Resource Names (ARNs) of the grants.</p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::ListDistributedGrants::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::ListDistributedGrants::set_filters): <p>Filters to scope the results. The following filters are supported:</p>  <ul>   <li> <p> <code>LicenseArn</code> </p> </li>   <li> <p> <code>GrantStatus</code> </p> </li>   <li> <p> <code>GranteePrincipalARN</code> </p> </li>   <li> <p> <code>ProductSKU</code> </p> </li>   <li> <p> <code>LicenseIssuerName</code> </p> </li>  </ul>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListDistributedGrants::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListDistributedGrants::set_next_token): <p>Token for the next set of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListDistributedGrants::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListDistributedGrants::set_max_results): <p>Maximum number of results to return in a single call.</p>
                            /// - On success, responds with [`ListDistributedGrantsOutput`](crate::output::ListDistributedGrantsOutput) with field(s):
    ///   - [`grants(Option<Vec<Grant>>)`](crate::output::ListDistributedGrantsOutput::grants): <p>Distributed grant details.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListDistributedGrantsOutput::next_token): <p>Token for the next set of results.</p>
                            /// - On failure, responds with [`SdkError<ListDistributedGrantsError>`](crate::error::ListDistributedGrantsError)
    pub fn list_distributed_grants(&self) -> crate::client::fluent_builders::ListDistributedGrants {
                                crate::client::fluent_builders::ListDistributedGrants::new(self.handle.clone())
                            }
}

