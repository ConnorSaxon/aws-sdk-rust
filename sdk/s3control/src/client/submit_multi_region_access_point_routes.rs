// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SubmitMultiRegionAccessPointRoutes`](crate::client::fluent_builders::SubmitMultiRegionAccessPointRoutes) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::SubmitMultiRegionAccessPointRoutes::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::SubmitMultiRegionAccessPointRoutes::set_account_id): <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    ///   - [`mrap(impl Into<String>)`](crate::client::fluent_builders::SubmitMultiRegionAccessPointRoutes::mrap) / [`set_mrap(Option<String>)`](crate::client::fluent_builders::SubmitMultiRegionAccessPointRoutes::set_mrap): <p>The Multi-Region Access Point ARN.</p>
    ///   - [`route_updates(Vec<MultiRegionAccessPointRoute>)`](crate::client::fluent_builders::SubmitMultiRegionAccessPointRoutes::route_updates) / [`set_route_updates(Option<Vec<MultiRegionAccessPointRoute>>)`](crate::client::fluent_builders::SubmitMultiRegionAccessPointRoutes::set_route_updates): <p>The different routes that make up the new route configuration. Active routes return a value of <code>100</code>, and passive routes return a value of <code>0</code>.</p>
                            /// - On success, responds with [`SubmitMultiRegionAccessPointRoutesOutput`](crate::output::SubmitMultiRegionAccessPointRoutesOutput)
                            /// - On failure, responds with [`SdkError<SubmitMultiRegionAccessPointRoutesError>`](crate::error::SubmitMultiRegionAccessPointRoutesError)
    pub fn submit_multi_region_access_point_routes(&self) -> crate::client::fluent_builders::SubmitMultiRegionAccessPointRoutes {
                                crate::client::fluent_builders::SubmitMultiRegionAccessPointRoutes::new(self.handle.clone())
                            }
}

