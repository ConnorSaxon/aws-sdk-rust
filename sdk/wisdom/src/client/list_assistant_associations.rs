// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListAssistantAssociations`](crate::client::fluent_builders::ListAssistantAssociations) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListAssistantAssociations::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListAssistantAssociations::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListAssistantAssociations::set_next_token): <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListAssistantAssociations::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListAssistantAssociations::set_max_results): <p>The maximum number of results to return per page.</p>
    ///   - [`assistant_id(impl Into<String>)`](crate::client::fluent_builders::ListAssistantAssociations::assistant_id) / [`set_assistant_id(Option<String>)`](crate::client::fluent_builders::ListAssistantAssociations::set_assistant_id): <p>The identifier of the Wisdom assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
                            /// - On success, responds with [`ListAssistantAssociationsOutput`](crate::output::ListAssistantAssociationsOutput) with field(s):
    ///   - [`assistant_association_summaries(Option<Vec<AssistantAssociationSummary>>)`](crate::output::ListAssistantAssociationsOutput::assistant_association_summaries): <p>Summary information about assistant associations.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListAssistantAssociationsOutput::next_token): <p>If there are additional results, this is the token for the next set of results.</p>
                            /// - On failure, responds with [`SdkError<ListAssistantAssociationsError>`](crate::error::ListAssistantAssociationsError)
    pub fn list_assistant_associations(&self) -> crate::client::fluent_builders::ListAssistantAssociations {
                                crate::client::fluent_builders::ListAssistantAssociations::new(self.handle.clone())
                            }
}

