// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListCodeSigningConfigs`](crate::client::fluent_builders::ListCodeSigningConfigs) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListCodeSigningConfigs::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListCodeSigningConfigs::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListCodeSigningConfigs::set_marker): <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    ///   - [`max_items(i32)`](crate::client::fluent_builders::ListCodeSigningConfigs::max_items) / [`set_max_items(Option<i32>)`](crate::client::fluent_builders::ListCodeSigningConfigs::set_max_items): <p>Maximum number of items to return.</p>
                            /// - On success, responds with [`ListCodeSigningConfigsOutput`](crate::output::ListCodeSigningConfigsOutput) with field(s):
    ///   - [`next_marker(Option<String>)`](crate::output::ListCodeSigningConfigsOutput::next_marker): <p>The pagination token that's included if more results are available.</p>
    ///   - [`code_signing_configs(Option<Vec<CodeSigningConfig>>)`](crate::output::ListCodeSigningConfigsOutput::code_signing_configs): <p>The code signing configurations</p>
                            /// - On failure, responds with [`SdkError<ListCodeSigningConfigsError>`](crate::error::ListCodeSigningConfigsError)
    pub fn list_code_signing_configs(&self) -> crate::client::fluent_builders::ListCodeSigningConfigs {
                                crate::client::fluent_builders::ListCodeSigningConfigs::new(self.handle.clone())
                            }
}

