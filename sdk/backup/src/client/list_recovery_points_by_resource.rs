// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListRecoveryPointsByResource`](crate::client::fluent_builders::ListRecoveryPointsByResource) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListRecoveryPointsByResource::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::ListRecoveryPointsByResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::ListRecoveryPointsByResource::set_resource_arn): <p>An ARN that uniquely identifies a resource. The format of the ARN depends on the resource type.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListRecoveryPointsByResource::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListRecoveryPointsByResource::set_next_token): <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListRecoveryPointsByResource::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListRecoveryPointsByResource::set_max_results): <p>The maximum number of items to be returned.</p> <note>   <p>Amazon RDS requires a value of at least 20.</p>  </note>
                            /// - On success, responds with [`ListRecoveryPointsByResourceOutput`](crate::output::ListRecoveryPointsByResourceOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListRecoveryPointsByResourceOutput::next_token): <p>The next item following a partial list of returned items. For example, if a request is made to return <code>maxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    ///   - [`recovery_points(Option<Vec<RecoveryPointByResource>>)`](crate::output::ListRecoveryPointsByResourceOutput::recovery_points): <p>An array of objects that contain detailed information about recovery points of the specified resource type.</p> <note>   <p>Only Amazon EFS and Amazon EC2 recovery points return BackupVaultName.</p>  </note>
                            /// - On failure, responds with [`SdkError<ListRecoveryPointsByResourceError>`](crate::error::ListRecoveryPointsByResourceError)
    pub fn list_recovery_points_by_resource(&self) -> crate::client::fluent_builders::ListRecoveryPointsByResource {
                                crate::client::fluent_builders::ListRecoveryPointsByResource::new(self.handle.clone())
                            }
}

