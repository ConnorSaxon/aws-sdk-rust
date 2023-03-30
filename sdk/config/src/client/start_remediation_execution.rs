// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartRemediationExecution`](crate::client::fluent_builders::StartRemediationExecution) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`config_rule_name(impl Into<String>)`](crate::client::fluent_builders::StartRemediationExecution::config_rule_name) / [`set_config_rule_name(Option<String>)`](crate::client::fluent_builders::StartRemediationExecution::set_config_rule_name): <p>The list of names of Config rules that you want to run remediation execution for.</p>
    ///   - [`resource_keys(Vec<ResourceKey>)`](crate::client::fluent_builders::StartRemediationExecution::resource_keys) / [`set_resource_keys(Option<Vec<ResourceKey>>)`](crate::client::fluent_builders::StartRemediationExecution::set_resource_keys): <p>A list of resource keys to be processed with the current request. Each element in the list consists of the resource type and resource ID. </p>
                            /// - On success, responds with [`StartRemediationExecutionOutput`](crate::output::StartRemediationExecutionOutput) with field(s):
    ///   - [`failure_message(Option<String>)`](crate::output::StartRemediationExecutionOutput::failure_message): <p>Returns a failure message. For example, the resource is already compliant.</p>
    ///   - [`failed_items(Option<Vec<ResourceKey>>)`](crate::output::StartRemediationExecutionOutput::failed_items): <p>For resources that have failed to start execution, the API returns a resource key object.</p>
                            /// - On failure, responds with [`SdkError<StartRemediationExecutionError>`](crate::error::StartRemediationExecutionError)
    pub fn start_remediation_execution(&self) -> crate::client::fluent_builders::StartRemediationExecution {
                                crate::client::fluent_builders::StartRemediationExecution::new(self.handle.clone())
                            }
}

