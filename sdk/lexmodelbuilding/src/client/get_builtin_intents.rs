// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetBuiltinIntents`](crate::client::fluent_builders::GetBuiltinIntents) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetBuiltinIntents::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`locale(Locale)`](crate::client::fluent_builders::GetBuiltinIntents::locale) / [`set_locale(Option<Locale>)`](crate::client::fluent_builders::GetBuiltinIntents::set_locale): <p>A list of locales that the intent supports.</p>
    ///   - [`signature_contains(impl Into<String>)`](crate::client::fluent_builders::GetBuiltinIntents::signature_contains) / [`set_signature_contains(Option<String>)`](crate::client::fluent_builders::GetBuiltinIntents::set_signature_contains): <p>Substring to match in built-in intent signatures. An intent will be returned if any part of its signature matches the substring. For example, "xyz" matches both "xyzabc" and "abcxyz." To find the signature for an intent, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills Kit</i>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetBuiltinIntents::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetBuiltinIntents::set_next_token): <p>A pagination token that fetches the next page of intents. If this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of intents, use the pagination token in the next request.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetBuiltinIntents::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetBuiltinIntents::set_max_results): <p>The maximum number of intents to return in the response. The default is 10.</p>
                            /// - On success, responds with [`GetBuiltinIntentsOutput`](crate::output::GetBuiltinIntentsOutput) with field(s):
    ///   - [`intents(Option<Vec<BuiltinIntentMetadata>>)`](crate::output::GetBuiltinIntentsOutput::intents): <p>An array of <code>builtinIntentMetadata</code> objects, one for each intent in the response.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetBuiltinIntentsOutput::next_token): <p>A pagination token that fetches the next page of intents. If the response to this API call is truncated, Amazon Lex returns a pagination token in the response. To fetch the next page of intents, specify the pagination token in the next request.</p>
                            /// - On failure, responds with [`SdkError<GetBuiltinIntentsError>`](crate::error::GetBuiltinIntentsError)
    pub fn get_builtin_intents(&self) -> crate::client::fluent_builders::GetBuiltinIntents {
                                crate::client::fluent_builders::GetBuiltinIntents::new(self.handle.clone())
                            }
}

