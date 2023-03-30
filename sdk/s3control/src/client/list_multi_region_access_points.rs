// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListMultiRegionAccessPoints`](crate::client::fluent_builders::ListMultiRegionAccessPoints) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListMultiRegionAccessPoints::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::ListMultiRegionAccessPoints::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::ListMultiRegionAccessPoints::set_account_id): <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListMultiRegionAccessPoints::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListMultiRegionAccessPoints::set_next_token): <p>Not currently used. Do not use this parameter.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListMultiRegionAccessPoints::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListMultiRegionAccessPoints::set_max_results): <p>Not currently used. Do not use this parameter.</p>
                            /// - On success, responds with [`ListMultiRegionAccessPointsOutput`](crate::output::ListMultiRegionAccessPointsOutput) with field(s):
    ///   - [`access_points(Option<Vec<MultiRegionAccessPointReport>>)`](crate::output::ListMultiRegionAccessPointsOutput::access_points): <p>The list of Multi-Region Access Points associated with the user.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListMultiRegionAccessPointsOutput::next_token): <p>If the specified bucket has more Multi-Region Access Points than can be returned in one call to this action, this field contains a continuation token. You can use this token tin subsequent calls to this action to retrieve additional Multi-Region Access Points.</p>
                            /// - On failure, responds with [`SdkError<ListMultiRegionAccessPointsError>`](crate::error::ListMultiRegionAccessPointsError)
    pub fn list_multi_region_access_points(&self) -> crate::client::fluent_builders::ListMultiRegionAccessPoints {
                                crate::client::fluent_builders::ListMultiRegionAccessPoints::new(self.handle.clone())
                            }
}

