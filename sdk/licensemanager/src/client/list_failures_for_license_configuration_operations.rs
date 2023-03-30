// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListFailuresForLicenseConfigurationOperations`](crate::client::fluent_builders::ListFailuresForLicenseConfigurationOperations) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`license_configuration_arn(impl Into<String>)`](crate::client::fluent_builders::ListFailuresForLicenseConfigurationOperations::license_configuration_arn) / [`set_license_configuration_arn(Option<String>)`](crate::client::fluent_builders::ListFailuresForLicenseConfigurationOperations::set_license_configuration_arn): <p>Amazon Resource Name of the license configuration.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListFailuresForLicenseConfigurationOperations::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListFailuresForLicenseConfigurationOperations::set_max_results): <p>Maximum number of results to return in a single call.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListFailuresForLicenseConfigurationOperations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListFailuresForLicenseConfigurationOperations::set_next_token): <p>Token for the next set of results.</p>
                            /// - On success, responds with [`ListFailuresForLicenseConfigurationOperationsOutput`](crate::output::ListFailuresForLicenseConfigurationOperationsOutput) with field(s):
    ///   - [`license_operation_failure_list(Option<Vec<LicenseOperationFailure>>)`](crate::output::ListFailuresForLicenseConfigurationOperationsOutput::license_operation_failure_list): <p>License configuration operations that failed.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListFailuresForLicenseConfigurationOperationsOutput::next_token): <p>Token for the next set of results.</p>
                            /// - On failure, responds with [`SdkError<ListFailuresForLicenseConfigurationOperationsError>`](crate::error::ListFailuresForLicenseConfigurationOperationsError)
    pub fn list_failures_for_license_configuration_operations(&self) -> crate::client::fluent_builders::ListFailuresForLicenseConfigurationOperations {
                                crate::client::fluent_builders::ListFailuresForLicenseConfigurationOperations::new(self.handle.clone())
                            }
}

