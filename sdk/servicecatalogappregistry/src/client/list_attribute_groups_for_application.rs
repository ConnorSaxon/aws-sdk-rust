// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListAttributeGroupsForApplication`](crate::client::fluent_builders::ListAttributeGroupsForApplication) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListAttributeGroupsForApplication::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`application(impl Into<String>)`](crate::client::fluent_builders::ListAttributeGroupsForApplication::application) / [`set_application(Option<String>)`](crate::client::fluent_builders::ListAttributeGroupsForApplication::set_application): <p>The name or ID of the application.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListAttributeGroupsForApplication::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListAttributeGroupsForApplication::set_next_token): <p>This token retrieves the next page of results after a previous API call.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListAttributeGroupsForApplication::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListAttributeGroupsForApplication::set_max_results): <p>The upper bound of the number of results to return. The value cannot exceed 25. If you omit this parameter, it defaults to 25. This value is optional.</p>
                            /// - On success, responds with [`ListAttributeGroupsForApplicationOutput`](crate::output::ListAttributeGroupsForApplicationOutput) with field(s):
    ///   - [`attribute_groups_details(Option<Vec<AttributeGroupDetails>>)`](crate::output::ListAttributeGroupsForApplicationOutput::attribute_groups_details): <p> The details related to a specific attribute group. </p>
    ///   - [`next_token(Option<String>)`](crate::output::ListAttributeGroupsForApplicationOutput::next_token): <p>The token to use to get the next page of results after a previous API call.</p>
                            /// - On failure, responds with [`SdkError<ListAttributeGroupsForApplicationError>`](crate::error::ListAttributeGroupsForApplicationError)
    pub fn list_attribute_groups_for_application(&self) -> crate::client::fluent_builders::ListAttributeGroupsForApplication {
                                crate::client::fluent_builders::ListAttributeGroupsForApplication::new(self.handle.clone())
                            }
}

