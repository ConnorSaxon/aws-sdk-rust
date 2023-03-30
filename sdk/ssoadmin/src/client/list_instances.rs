// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListInstances`](crate::client::fluent_builders::ListInstances) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListInstances::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListInstances::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListInstances::set_max_results): <p>The maximum number of results to display for the instance.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListInstances::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListInstances::set_next_token): <p>The pagination token for the list API. Initially the value is null. Use the output of previous API calls to make subsequent calls.</p>
                            /// - On success, responds with [`ListInstancesOutput`](crate::output::ListInstancesOutput) with field(s):
    ///   - [`instances(Option<Vec<InstanceMetadata>>)`](crate::output::ListInstancesOutput::instances): <p>Lists the IAM Identity Center instances that the caller has access to.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListInstancesOutput::next_token): <p>The pagination token for the list API. Initially the value is null. Use the output of previous API calls to make subsequent calls.</p>
                            /// - On failure, responds with [`SdkError<ListInstancesError>`](crate::error::ListInstancesError)
    pub fn list_instances(&self) -> crate::client::fluent_builders::ListInstances {
                                crate::client::fluent_builders::ListInstances::new(self.handle.clone())
                            }
}

