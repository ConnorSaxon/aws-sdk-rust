// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetVocabularyFilter`](crate::client::fluent_builders::GetVocabularyFilter) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`vocabulary_filter_name(impl Into<String>)`](crate::client::fluent_builders::GetVocabularyFilter::vocabulary_filter_name) / [`set_vocabulary_filter_name(Option<String>)`](crate::client::fluent_builders::GetVocabularyFilter::set_vocabulary_filter_name): <p>The name of the custom vocabulary filter you want information about. Custom vocabulary filter names are case sensitive.</p>
                            /// - On success, responds with [`GetVocabularyFilterOutput`](crate::output::GetVocabularyFilterOutput) with field(s):
    ///   - [`vocabulary_filter_name(Option<String>)`](crate::output::GetVocabularyFilterOutput::vocabulary_filter_name): <p>The name of the custom vocabulary filter you requested information about.</p>
    ///   - [`language_code(Option<LanguageCode>)`](crate::output::GetVocabularyFilterOutput::language_code): <p>The language code you selected for your custom vocabulary filter.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::output::GetVocabularyFilterOutput::last_modified_time): <p>The date and time the specified custom vocabulary filter was last modified.</p>  <p>Timestamps are in the format <code>YYYY-MM-DD'T'HH:MM:SS.SSSSSS-UTC</code>. For example, <code>2022-05-04T12:32:58.761000-07:00</code> represents 12:32 PM UTC-7 on May 4, 2022.</p>
    ///   - [`download_uri(Option<String>)`](crate::output::GetVocabularyFilterOutput::download_uri): <p>The Amazon S3 location where the custom vocabulary filter is stored; use this URI to view or download the custom vocabulary filter.</p>
                            /// - On failure, responds with [`SdkError<GetVocabularyFilterError>`](crate::error::GetVocabularyFilterError)
    pub fn get_vocabulary_filter(&self) -> crate::client::fluent_builders::GetVocabularyFilter {
                                crate::client::fluent_builders::GetVocabularyFilter::new(self.handle.clone())
                            }
}

