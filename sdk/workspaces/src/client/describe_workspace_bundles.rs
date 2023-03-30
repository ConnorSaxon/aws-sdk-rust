// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeWorkspaceBundles`](crate::client::fluent_builders::DescribeWorkspaceBundles) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeWorkspaceBundles::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bundle_ids(Vec<String>)`](crate::client::fluent_builders::DescribeWorkspaceBundles::bundle_ids) / [`set_bundle_ids(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeWorkspaceBundles::set_bundle_ids): <p>The identifiers of the bundles. You cannot combine this parameter with any other filter.</p>
    ///   - [`owner(impl Into<String>)`](crate::client::fluent_builders::DescribeWorkspaceBundles::owner) / [`set_owner(Option<String>)`](crate::client::fluent_builders::DescribeWorkspaceBundles::set_owner): <p>The owner of the bundles. You cannot combine this parameter with any other filter.</p>  <p>To describe the bundles provided by Amazon Web Services, specify <code>AMAZON</code>. To describe the bundles that belong to your account, don't specify a value.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeWorkspaceBundles::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeWorkspaceBundles::set_next_token): <p>The token for the next set of results. (You received this token from a previous call.)</p>
                            /// - On success, responds with [`DescribeWorkspaceBundlesOutput`](crate::output::DescribeWorkspaceBundlesOutput) with field(s):
    ///   - [`bundles(Option<Vec<WorkspaceBundle>>)`](crate::output::DescribeWorkspaceBundlesOutput::bundles): <p>Information about the bundles.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeWorkspaceBundlesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return. This token is valid for one day and must be used within that time frame.</p>
                            /// - On failure, responds with [`SdkError<DescribeWorkspaceBundlesError>`](crate::error::DescribeWorkspaceBundlesError)
    pub fn describe_workspace_bundles(&self) -> crate::client::fluent_builders::DescribeWorkspaceBundles {
                                crate::client::fluent_builders::DescribeWorkspaceBundles::new(self.handle.clone())
                            }
}

