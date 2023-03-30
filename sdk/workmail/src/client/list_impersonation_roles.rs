// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListImpersonationRoles`](crate::client::fluent_builders::ListImpersonationRoles) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListImpersonationRoles::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`organization_id(impl Into<String>)`](crate::client::fluent_builders::ListImpersonationRoles::organization_id) / [`set_organization_id(Option<String>)`](crate::client::fluent_builders::ListImpersonationRoles::set_organization_id): <p>The WorkMail organization to which the listed impersonation roles belong.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListImpersonationRoles::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListImpersonationRoles::set_next_token): <p>The token used to retrieve the next page of results. The first call doesn't require a token.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListImpersonationRoles::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListImpersonationRoles::set_max_results): <p>The maximum number of results returned in a single call.</p>
                            /// - On success, responds with [`ListImpersonationRolesOutput`](crate::output::ListImpersonationRolesOutput) with field(s):
    ///   - [`roles(Option<Vec<ImpersonationRole>>)`](crate::output::ListImpersonationRolesOutput::roles): <p>The list of impersonation roles under the given WorkMail organization.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListImpersonationRolesOutput::next_token): <p>The token to retrieve the next page of results. The value is <code>null</code> when there are no results to return.</p>
                            /// - On failure, responds with [`SdkError<ListImpersonationRolesError>`](crate::error::ListImpersonationRolesError)
    pub fn list_impersonation_roles(&self) -> crate::client::fluent_builders::ListImpersonationRoles {
                                crate::client::fluent_builders::ListImpersonationRoles::new(self.handle.clone())
                            }
}

