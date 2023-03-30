// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListAppInstanceAdmins`](crate::client::fluent_builders::ListAppInstanceAdmins) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListAppInstanceAdmins::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`app_instance_arn(impl Into<String>)`](crate::client::fluent_builders::ListAppInstanceAdmins::app_instance_arn) / [`set_app_instance_arn(Option<String>)`](crate::client::fluent_builders::ListAppInstanceAdmins::set_app_instance_arn): <p>The ARN of the <code>AppInstance</code>.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListAppInstanceAdmins::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListAppInstanceAdmins::set_max_results): <p>The maximum number of administrators that you want to return.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListAppInstanceAdmins::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListAppInstanceAdmins::set_next_token): <p>The token returned from previous API requests until the number of administrators is reached.</p>
                            /// - On success, responds with [`ListAppInstanceAdminsOutput`](crate::output::ListAppInstanceAdminsOutput) with field(s):
    ///   - [`app_instance_arn(Option<String>)`](crate::output::ListAppInstanceAdminsOutput::app_instance_arn): <p>The ARN of the <code>AppInstance</code>.</p>
    ///   - [`app_instance_admins(Option<Vec<AppInstanceAdminSummary>>)`](crate::output::ListAppInstanceAdminsOutput::app_instance_admins): <p>The information for each administrator.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListAppInstanceAdminsOutput::next_token): <p>The token returned from previous API requests until the number of administrators is reached.</p>
                            /// - On failure, responds with [`SdkError<ListAppInstanceAdminsError>`](crate::error::ListAppInstanceAdminsError)
    pub fn list_app_instance_admins(&self) -> crate::client::fluent_builders::ListAppInstanceAdmins {
                                crate::client::fluent_builders::ListAppInstanceAdmins::new(self.handle.clone())
                            }
}

