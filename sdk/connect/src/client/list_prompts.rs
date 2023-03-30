// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListPrompts`](crate::client::fluent_builders::ListPrompts) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListPrompts::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::ListPrompts::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::ListPrompts::set_instance_id): <p>The identifier of the Amazon Connect instance.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListPrompts::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListPrompts::set_next_token): <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListPrompts::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListPrompts::set_max_results): <p>The maximum number of results to return per page. The default MaxResult size is 100.</p>
                            /// - On success, responds with [`ListPromptsOutput`](crate::output::ListPromptsOutput) with field(s):
    ///   - [`prompt_summary_list(Option<Vec<PromptSummary>>)`](crate::output::ListPromptsOutput::prompt_summary_list): <p>Information about the prompts.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListPromptsOutput::next_token): <p>If there are additional results, this is the token for the next set of results.</p>
                            /// - On failure, responds with [`SdkError<ListPromptsError>`](crate::error::ListPromptsError)
    pub fn list_prompts(&self) -> crate::client::fluent_builders::ListPrompts {
                                crate::client::fluent_builders::ListPrompts::new(self.handle.clone())
                            }
}

