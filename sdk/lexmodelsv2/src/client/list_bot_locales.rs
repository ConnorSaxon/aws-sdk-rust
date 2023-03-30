// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListBotLocales`](crate::client::fluent_builders::ListBotLocales) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListBotLocales::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bot_id(impl Into<String>)`](crate::client::fluent_builders::ListBotLocales::bot_id) / [`set_bot_id(Option<String>)`](crate::client::fluent_builders::ListBotLocales::set_bot_id): <p>The identifier of the bot to list locales for.</p>
    ///   - [`bot_version(impl Into<String>)`](crate::client::fluent_builders::ListBotLocales::bot_version) / [`set_bot_version(Option<String>)`](crate::client::fluent_builders::ListBotLocales::set_bot_version): <p>The version of the bot to list locales for.</p>
    ///   - [`sort_by(BotLocaleSortBy)`](crate::client::fluent_builders::ListBotLocales::sort_by) / [`set_sort_by(Option<BotLocaleSortBy>)`](crate::client::fluent_builders::ListBotLocales::set_sort_by): <p>Specifies sorting parameters for the list of locales. You can sort by locale name in ascending or descending order.</p>
    ///   - [`filters(Vec<BotLocaleFilter>)`](crate::client::fluent_builders::ListBotLocales::filters) / [`set_filters(Option<Vec<BotLocaleFilter>>)`](crate::client::fluent_builders::ListBotLocales::set_filters): <p>Provides the specification for a filter used to limit the response to only those locales that match the filter specification. You can only specify one filter and one value to filter on.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListBotLocales::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListBotLocales::set_max_results): <p>The maximum number of aliases to return in each page of results. If there are fewer results than the max page size, only the actual number of results are returned.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListBotLocales::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListBotLocales::set_next_token): <p>If the response from the <code>ListBotLocales</code> operation contains more results than specified in the <code>maxResults</code> parameter, a token is returned in the response. Use that token as the <code>nextToken</code> parameter to return the next page of results. </p>
                            /// - On success, responds with [`ListBotLocalesOutput`](crate::output::ListBotLocalesOutput) with field(s):
    ///   - [`bot_id(Option<String>)`](crate::output::ListBotLocalesOutput::bot_id): <p>The identifier of the bot to list locales for.</p>
    ///   - [`bot_version(Option<String>)`](crate::output::ListBotLocalesOutput::bot_version): <p>The version of the bot.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListBotLocalesOutput::next_token): <p>A token that indicates whether there are more results to return in a response to the <code>ListBotLocales</code> operation. If the <code>nextToken</code> field is present, you send the contents as the <code>nextToken</code> parameter of a <code>ListBotLocales</code> operation request to get the next page of results.</p>
    ///   - [`bot_locale_summaries(Option<Vec<BotLocaleSummary>>)`](crate::output::ListBotLocalesOutput::bot_locale_summaries): <p>Summary information for the locales that meet the filter criteria specified in the request. The length of the list is specified in the <code>maxResults</code> parameter of the request. If there are more locales available, the <code>nextToken</code> field contains a token to get the next page of results.</p>
                            /// - On failure, responds with [`SdkError<ListBotLocalesError>`](crate::error::ListBotLocalesError)
    pub fn list_bot_locales(&self) -> crate::client::fluent_builders::ListBotLocales {
                                crate::client::fluent_builders::ListBotLocales::new(self.handle.clone())
                            }
}

