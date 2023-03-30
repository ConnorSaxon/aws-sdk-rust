// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListThemes`](crate::client::fluent_builders::ListThemes) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListThemes::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::client::fluent_builders::ListThemes::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::client::fluent_builders::ListThemes::set_aws_account_id): <p>The ID of the Amazon Web Services account that contains the themes that you're listing.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListThemes::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListThemes::set_next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListThemes::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListThemes::set_max_results): <p>The maximum number of results to be returned per request.</p>
    ///   - [`r#type(ThemeType)`](crate::client::fluent_builders::ListThemes::type) / [`set_type(Option<ThemeType>)`](crate::client::fluent_builders::ListThemes::set_type): <p>The type of themes that you want to list. Valid options include the following:</p>  <ul>   <li> <p> <code>ALL (default)</code>- Display all existing themes.</p> </li>   <li> <p> <code>CUSTOM</code> - Display only the themes created by people using Amazon QuickSight.</p> </li>   <li> <p> <code>QUICKSIGHT</code> - Display only the starting themes defined by Amazon QuickSight.</p> </li>  </ul>
                            /// - On success, responds with [`ListThemesOutput`](crate::output::ListThemesOutput) with field(s):
    ///   - [`theme_summary_list(Option<Vec<ThemeSummary>>)`](crate::output::ListThemesOutput::theme_summary_list): <p>Information about the themes in the list.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListThemesOutput::next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`status(i32)`](crate::output::ListThemesOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`request_id(Option<String>)`](crate::output::ListThemesOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
                            /// - On failure, responds with [`SdkError<ListThemesError>`](crate::error::ListThemesError)
    pub fn list_themes(&self) -> crate::client::fluent_builders::ListThemes {
                                crate::client::fluent_builders::ListThemes::new(self.handle.clone())
                            }
}

