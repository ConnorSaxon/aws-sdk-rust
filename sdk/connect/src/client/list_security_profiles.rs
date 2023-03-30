// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListSecurityProfiles`](crate::client::fluent_builders::ListSecurityProfiles) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListSecurityProfiles::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::ListSecurityProfiles::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::ListSecurityProfiles::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListSecurityProfiles::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListSecurityProfiles::set_next_token): <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListSecurityProfiles::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListSecurityProfiles::set_max_results): <p>The maximum number of results to return per page. The default MaxResult size is 100.</p>
                            /// - On success, responds with [`ListSecurityProfilesOutput`](crate::output::ListSecurityProfilesOutput) with field(s):
    ///   - [`security_profile_summary_list(Option<Vec<SecurityProfileSummary>>)`](crate::output::ListSecurityProfilesOutput::security_profile_summary_list): <p>Information about the security profiles.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListSecurityProfilesOutput::next_token): <p>If there are additional results, this is the token for the next set of results.</p>
                            /// - On failure, responds with [`SdkError<ListSecurityProfilesError>`](crate::error::ListSecurityProfilesError)
    pub fn list_security_profiles(&self) -> crate::client::fluent_builders::ListSecurityProfiles {
                                crate::client::fluent_builders::ListSecurityProfiles::new(self.handle.clone())
                            }
}

