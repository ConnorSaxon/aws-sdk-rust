// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListResolverConfigs`](crate::client::fluent_builders::ListResolverConfigs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListResolverConfigs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListResolverConfigs::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListResolverConfigs::set_max_results): <p>The maximum number of Resolver configurations that you want to return in the response to a <code>ListResolverConfigs</code> request. If you don't specify a value for <code>MaxResults</code>, up to 100 Resolver configurations are returned.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListResolverConfigs::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListResolverConfigs::set_next_token): <p>(Optional) If the current Amazon Web Services account has more than <code>MaxResults</code> Resolver configurations, use <code>NextToken</code> to get the second and subsequent pages of results.</p>  <p>For the first <code>ListResolverConfigs</code> request, omit this value.</p>  <p>For the second and subsequent requests, get the value of <code>NextToken</code> from the previous response and specify that value for <code>NextToken</code> in the request.</p>
                            /// - On success, responds with [`ListResolverConfigsOutput`](crate::output::ListResolverConfigsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListResolverConfigsOutput::next_token): <p>If a response includes the last of the Resolver configurations that are associated with the current Amazon Web Services account, <code>NextToken</code> doesn't appear in the response.</p>  <p>If a response doesn't include the last of the configurations, you can get more configurations by submitting another <code>ListResolverConfigs</code> request. Get the value of <code>NextToken</code> that Amazon Route 53 returned in the previous response and include it in <code>NextToken</code> in the next request.</p>
    ///   - [`resolver_configs(Option<Vec<ResolverConfig>>)`](crate::output::ListResolverConfigsOutput::resolver_configs): <p>An array that contains one <code>ResolverConfigs</code> element for each Resolver configuration that is associated with the current Amazon Web Services account.</p>
                            /// - On failure, responds with [`SdkError<ListResolverConfigsError>`](crate::error::ListResolverConfigsError)
    pub fn list_resolver_configs(&self) -> crate::client::fluent_builders::ListResolverConfigs {
                                crate::client::fluent_builders::ListResolverConfigs::new(self.handle.clone())
                            }
}

