// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`EnableReachabilityAnalyzerOrganizationSharing`](crate::client::fluent_builders::EnableReachabilityAnalyzerOrganizationSharing) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::EnableReachabilityAnalyzerOrganizationSharing::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::EnableReachabilityAnalyzerOrganizationSharing::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`EnableReachabilityAnalyzerOrganizationSharingOutput`](crate::output::EnableReachabilityAnalyzerOrganizationSharingOutput) with field(s):
    ///   - [`return_value(Option<bool>)`](crate::output::EnableReachabilityAnalyzerOrganizationSharingOutput::return_value): <p>Returns <code>true</code> if the request succeeds; otherwise, returns an error.</p>
                            /// - On failure, responds with [`SdkError<EnableReachabilityAnalyzerOrganizationSharingError>`](crate::error::EnableReachabilityAnalyzerOrganizationSharingError)
    pub fn enable_reachability_analyzer_organization_sharing(&self) -> crate::client::fluent_builders::EnableReachabilityAnalyzerOrganizationSharing {
                                crate::client::fluent_builders::EnableReachabilityAnalyzerOrganizationSharing::new(self.handle.clone())
                            }
}

