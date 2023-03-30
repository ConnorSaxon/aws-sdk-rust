// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CancelBundleTask`](crate::client::fluent_builders::CancelBundleTask) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bundle_id(impl Into<String>)`](crate::client::fluent_builders::CancelBundleTask::bundle_id) / [`set_bundle_id(Option<String>)`](crate::client::fluent_builders::CancelBundleTask::set_bundle_id): <p>The ID of the bundle task.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CancelBundleTask::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CancelBundleTask::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
                            /// - On success, responds with [`CancelBundleTaskOutput`](crate::output::CancelBundleTaskOutput) with field(s):
    ///   - [`bundle_task(Option<BundleTask>)`](crate::output::CancelBundleTaskOutput::bundle_task): <p>Information about the bundle task.</p>
                            /// - On failure, responds with [`SdkError<CancelBundleTaskError>`](crate::error::CancelBundleTaskError)
    pub fn cancel_bundle_task(&self) -> crate::client::fluent_builders::CancelBundleTask {
                                crate::client::fluent_builders::CancelBundleTask::new(self.handle.clone())
                            }
}

