// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListCustomVocabularyItems`](crate::client::fluent_builders::ListCustomVocabularyItems) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListCustomVocabularyItems::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bot_id(impl Into<String>)`](crate::client::fluent_builders::ListCustomVocabularyItems::bot_id) / [`set_bot_id(Option<String>)`](crate::client::fluent_builders::ListCustomVocabularyItems::set_bot_id): <p>The unique identifier of the bot to the list custom vocabulary request.</p>
    ///   - [`bot_version(impl Into<String>)`](crate::client::fluent_builders::ListCustomVocabularyItems::bot_version) / [`set_bot_version(Option<String>)`](crate::client::fluent_builders::ListCustomVocabularyItems::set_bot_version): <p>The bot version of the bot to the list custom vocabulary request.</p>
    ///   - [`locale_id(impl Into<String>)`](crate::client::fluent_builders::ListCustomVocabularyItems::locale_id) / [`set_locale_id(Option<String>)`](crate::client::fluent_builders::ListCustomVocabularyItems::set_locale_id): <p>The locale identifier of the bot to the list custom vocabulary request.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListCustomVocabularyItems::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListCustomVocabularyItems::set_max_results): <p>The maximum results to the list custom vocabulary request.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListCustomVocabularyItems::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListCustomVocabularyItems::set_next_token): <p>The nextToken identifier to the list custom vocabulary request.</p>
                            /// - On success, responds with [`ListCustomVocabularyItemsOutput`](crate::output::ListCustomVocabularyItemsOutput) with field(s):
    ///   - [`bot_id(Option<String>)`](crate::output::ListCustomVocabularyItemsOutput::bot_id): <p>The unique identifier of the bot to the list custom vocabulary response.</p>
    ///   - [`bot_version(Option<String>)`](crate::output::ListCustomVocabularyItemsOutput::bot_version): <p>The bot version of the bot to the list custom vocabulary response.</p>
    ///   - [`locale_id(Option<String>)`](crate::output::ListCustomVocabularyItemsOutput::locale_id): <p>The locale identifier of the bot to the list custom vocabulary response.</p>
    ///   - [`custom_vocabulary_items(Option<Vec<CustomVocabularyItem>>)`](crate::output::ListCustomVocabularyItemsOutput::custom_vocabulary_items): <p>The custom vocabulary items from the list custom vocabulary response.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListCustomVocabularyItemsOutput::next_token): <p>The nextToken identifier to the list custom vocabulary response.</p>
                            /// - On failure, responds with [`SdkError<ListCustomVocabularyItemsError>`](crate::error::ListCustomVocabularyItemsError)
    pub fn list_custom_vocabulary_items(&self) -> crate::client::fluent_builders::ListCustomVocabularyItems {
                                crate::client::fluent_builders::ListCustomVocabularyItems::new(self.handle.clone())
                            }
}

