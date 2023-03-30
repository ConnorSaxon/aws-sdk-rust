// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetBotAliases`](crate::client::fluent_builders::GetBotAliases) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetBotAliases::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bot_name(impl Into<String>)`](crate::client::fluent_builders::GetBotAliases::bot_name) / [`set_bot_name(Option<String>)`](crate::client::fluent_builders::GetBotAliases::set_bot_name): <p>The name of the bot.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetBotAliases::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetBotAliases::set_next_token): <p>A pagination token for fetching the next page of aliases. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of aliases, specify the pagination token in the next request. </p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetBotAliases::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetBotAliases::set_max_results): <p>The maximum number of aliases to return in the response. The default is 50. . </p>
    ///   - [`name_contains(impl Into<String>)`](crate::client::fluent_builders::GetBotAliases::name_contains) / [`set_name_contains(Option<String>)`](crate::client::fluent_builders::GetBotAliases::set_name_contains): <p>Substring to match in bot alias names. An alias will be returned if any part of its name matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz."</p>
                            /// - On success, responds with [`GetBotAliasesOutput`](crate::output::GetBotAliasesOutput) with field(s):
    ///   - [`bot_aliases(Option<Vec<BotAliasMetadata>>)`](crate::output::GetBotAliasesOutput::bot_aliases): <p>An array of <code>BotAliasMetadata</code> objects, each describing a bot alias.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetBotAliasesOutput::next_token): <p>A pagination token for fetching next page of aliases. If the response to this call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of aliases, specify the pagination token in the next request. </p>
                            /// - On failure, responds with [`SdkError<GetBotAliasesError>`](crate::error::GetBotAliasesError)
    pub fn get_bot_aliases(&self) -> crate::client::fluent_builders::GetBotAliases {
                                crate::client::fluent_builders::GetBotAliases::new(self.handle.clone())
                            }
}

