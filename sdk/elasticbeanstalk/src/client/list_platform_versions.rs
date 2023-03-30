// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPlatformVersions`](crate::client::fluent_builders::ListPlatformVersions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPlatformVersions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`filters(Vec<PlatformFilter>)`](crate::client::fluent_builders::ListPlatformVersions::filters) / [`set_filters(Option<Vec<PlatformFilter>>)`](crate::client::fluent_builders::ListPlatformVersions::set_filters): <p>Criteria for restricting the resulting list of platform versions. The filter is interpreted as a logical conjunction (AND) of the separate <code>PlatformFilter</code> terms.</p>
    ///   - [`max_records(i32)`](crate::client::fluent_builders::ListPlatformVersions::max_records) / [`set_max_records(Option<i32>)`](crate::client::fluent_builders::ListPlatformVersions::set_max_records): <p>The maximum number of platform version values returned in one call.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPlatformVersions::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPlatformVersions::set_next_token): <p>For a paginated request. Specify a token from a previous response page to retrieve the next response page. All other parameter values must be identical to the ones specified in the initial request.</p>  <p>If no <code>NextToken</code> is specified, the first page is retrieved.</p>
                            /// - On success, responds with [`ListPlatformVersionsOutput`](crate::output::ListPlatformVersionsOutput) with field(s):
    ///   - [`platform_summary_list(Option<Vec<PlatformSummary>>)`](crate::output::ListPlatformVersionsOutput::platform_summary_list): <p>Summary information about the platform versions.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListPlatformVersionsOutput::next_token): <p>In a paginated request, if this value isn't <code>null</code>, it's the token that you can pass in a subsequent request to get the next response page.</p>
                            /// - On failure, responds with [`SdkError<ListPlatformVersionsError>`](crate::error::ListPlatformVersionsError)
    pub fn list_platform_versions(&self) -> crate::client::fluent_builders::ListPlatformVersions {
                                crate::client::fluent_builders::ListPlatformVersions::new(self.handle.clone())
                            }
}

