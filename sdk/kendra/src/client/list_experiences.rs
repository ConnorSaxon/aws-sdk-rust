// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListExperiences`](crate::client::fluent_builders::ListExperiences) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListExperiences::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`index_id(impl Into<String>)`](crate::client::fluent_builders::ListExperiences::index_id) / [`set_index_id(Option<String>)`](crate::client::fluent_builders::ListExperiences::set_index_id): <p>The identifier of the index for your Amazon Kendra experience.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListExperiences::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListExperiences::set_next_token): <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Kendra returns a pagination token in the response. You can use this pagination token to retrieve the next set of Amazon Kendra experiences.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListExperiences::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListExperiences::set_max_results): <p>The maximum number of returned Amazon Kendra experiences.</p>
                            /// - On success, responds with [`ListExperiencesOutput`](crate::output::ListExperiencesOutput) with field(s):
    ///   - [`summary_items(Option<Vec<ExperiencesSummary>>)`](crate::output::ListExperiencesOutput::summary_items): <p>An array of summary information for one or more Amazon Kendra experiences.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListExperiencesOutput::next_token): <p>If the response is truncated, Amazon Kendra returns this token, which you can use in a later request to retrieve the next set of Amazon Kendra experiences.</p>
                            /// - On failure, responds with [`SdkError<ListExperiencesError>`](crate::error::ListExperiencesError)
    pub fn list_experiences(&self) -> crate::client::fluent_builders::ListExperiences {
                                crate::client::fluent_builders::ListExperiences::new(self.handle.clone())
                            }
}

