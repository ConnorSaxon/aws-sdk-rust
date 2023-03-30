// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListInstanceTypeDetails`](crate::client::fluent_builders::ListInstanceTypeDetails) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListInstanceTypeDetails::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`engine_version(impl Into<String>)`](crate::client::fluent_builders::ListInstanceTypeDetails::engine_version) / [`set_engine_version(Option<String>)`](crate::client::fluent_builders::ListInstanceTypeDetails::set_engine_version): <p>Version of OpenSearch or Elasticsearch, in the format Elasticsearch_X.Y or OpenSearch_X.Y. Defaults to the latest version of OpenSearch.</p>
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::ListInstanceTypeDetails::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::ListInstanceTypeDetails::set_domain_name): <p>Name of the domain to list instance type details for.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListInstanceTypeDetails::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListInstanceTypeDetails::set_max_results): <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListInstanceTypeDetails::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListInstanceTypeDetails::set_next_token): <p>If your initial <code>ListInstanceTypeDetails</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListInstanceTypeDetails</code> operations, which returns results in the next page.</p>
                            /// - On success, responds with [`ListInstanceTypeDetailsOutput`](crate::output::ListInstanceTypeDetailsOutput) with field(s):
    ///   - [`instance_type_details(Option<Vec<InstanceTypeDetails>>)`](crate::output::ListInstanceTypeDetailsOutput::instance_type_details): <p>Lists all supported instance types and features for the given OpenSearch or Elasticsearch version.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListInstanceTypeDetailsOutput::next_token): <p>When <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
                            /// - On failure, responds with [`SdkError<ListInstanceTypeDetailsError>`](crate::error::ListInstanceTypeDetailsError)
    pub fn list_instance_type_details(&self) -> crate::client::fluent_builders::ListInstanceTypeDetails {
                                crate::client::fluent_builders::ListInstanceTypeDetails::new(self.handle.clone())
                            }
}

