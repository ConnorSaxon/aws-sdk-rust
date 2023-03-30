// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListDomains`](crate::client::fluent_builders::ListDomains) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListDomains::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`filter_conditions(Vec<FilterCondition>)`](crate::client::fluent_builders::ListDomains::filter_conditions) / [`set_filter_conditions(Option<Vec<FilterCondition>>)`](crate::client::fluent_builders::ListDomains::set_filter_conditions): <p>A complex type that contains information about the filters applied during the <code>ListDomains</code> request. The filter conditions can include domain name and domain expiration.</p>
    ///   - [`sort_condition(SortCondition)`](crate::client::fluent_builders::ListDomains::sort_condition) / [`set_sort_condition(Option<SortCondition>)`](crate::client::fluent_builders::ListDomains::set_sort_condition): <p>A complex type that contains information about the requested ordering of domains in the returned list.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListDomains::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListDomains::set_marker): <p>For an initial request for a list of domains, omit this element. If the number of domains that are associated with the current Amazon Web Services account is greater than the value that you specified for <code>MaxItems</code>, you can use <code>Marker</code> to return additional domains. Get the value of <code>NextPageMarker</code> from the previous response, and submit another request that includes the value of <code>NextPageMarker</code> in the <code>Marker</code> element.</p>  <p>Constraints: The marker must match the value specified in the previous request.</p>
    ///   - [`max_items(i32)`](crate::client::fluent_builders::ListDomains::max_items) / [`set_max_items(Option<i32>)`](crate::client::fluent_builders::ListDomains::set_max_items): <p>Number of domains to be returned.</p>  <p>Default: 20</p>
                            /// - On success, responds with [`ListDomainsOutput`](crate::output::ListDomainsOutput) with field(s):
    ///   - [`domains(Option<Vec<DomainSummary>>)`](crate::output::ListDomainsOutput::domains): <p>A list of domains.</p>
    ///   - [`next_page_marker(Option<String>)`](crate::output::ListDomainsOutput::next_page_marker): <p>If there are more domains than you specified for <code>MaxItems</code> in the request, submit another request and include the value of <code>NextPageMarker</code> in the value of <code>Marker</code>.</p>
                            /// - On failure, responds with [`SdkError<ListDomainsError>`](crate::error::ListDomainsError)
    pub fn list_domains(&self) -> crate::client::fluent_builders::ListDomains {
                                crate::client::fluent_builders::ListDomains::new(self.handle.clone())
                            }
}

