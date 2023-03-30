// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SearchSecurityProfiles`](crate::client::fluent_builders::SearchSecurityProfiles) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::SearchSecurityProfiles::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::SearchSecurityProfiles::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::SearchSecurityProfiles::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::SearchSecurityProfiles::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::SearchSecurityProfiles::set_next_token): <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::SearchSecurityProfiles::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::SearchSecurityProfiles::set_max_results): <p>The maximum number of results to return per page.</p>
    ///   - [`search_criteria(SecurityProfileSearchCriteria)`](crate::client::fluent_builders::SearchSecurityProfiles::search_criteria) / [`set_search_criteria(Option<SecurityProfileSearchCriteria>)`](crate::client::fluent_builders::SearchSecurityProfiles::set_search_criteria): <p>The search criteria to be used to return security profiles. </p> <note>   <p>The <code>name</code> field support "contains" queries with a minimum of 2 characters and maximum of 25 characters. Any queries with character lengths outside of this range will throw invalid results.</p>  </note> <note>   <p>The currently supported value for <code>FieldName</code>: <code>name</code> </p>  </note>
    ///   - [`search_filter(SecurityProfilesSearchFilter)`](crate::client::fluent_builders::SearchSecurityProfiles::search_filter) / [`set_search_filter(Option<SecurityProfilesSearchFilter>)`](crate::client::fluent_builders::SearchSecurityProfiles::set_search_filter): <p>Filters to be applied to search results.</p>
                            /// - On success, responds with [`SearchSecurityProfilesOutput`](crate::output::SearchSecurityProfilesOutput) with field(s):
    ///   - [`security_profiles(Option<Vec<SecurityProfileSearchSummary>>)`](crate::output::SearchSecurityProfilesOutput::security_profiles): <p>Information about the security profiles.</p>
    ///   - [`next_token(Option<String>)`](crate::output::SearchSecurityProfilesOutput::next_token): <p>If there are additional results, this is the token for the next set of results.</p>
    ///   - [`approximate_total_count(Option<i64>)`](crate::output::SearchSecurityProfilesOutput::approximate_total_count): <p>The total number of security profiles which matched your search query.</p>
                            /// - On failure, responds with [`SdkError<SearchSecurityProfilesError>`](crate::error::SearchSecurityProfilesError)
    pub fn search_security_profiles(&self) -> crate::client::fluent_builders::SearchSecurityProfiles {
                                crate::client::fluent_builders::SearchSecurityProfiles::new(self.handle.clone())
                            }
}

