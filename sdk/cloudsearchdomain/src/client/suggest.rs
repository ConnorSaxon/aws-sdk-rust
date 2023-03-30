// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`Suggest`](crate::client::fluent_builders::Suggest) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`query(impl Into<String>)`](crate::client::fluent_builders::Suggest::query) / [`set_query(Option<String>)`](crate::client::fluent_builders::Suggest::set_query): <p>Specifies the string for which you want to get suggestions.</p>
    ///   - [`suggester(impl Into<String>)`](crate::client::fluent_builders::Suggest::suggester) / [`set_suggester(Option<String>)`](crate::client::fluent_builders::Suggest::set_suggester): <p>Specifies the name of the suggester to use to find suggested matches.</p>
    ///   - [`size(i64)`](crate::client::fluent_builders::Suggest::size) / [`set_size(i64)`](crate::client::fluent_builders::Suggest::set_size): <p>Specifies the maximum number of suggestions to return. </p>
                            /// - On success, responds with [`SuggestOutput`](crate::output::SuggestOutput) with field(s):
    ///   - [`status(Option<SuggestStatus>)`](crate::output::SuggestOutput::status): <p>The status of a <code>SuggestRequest</code>. Contains the resource ID (<code>rid</code>) and how long it took to process the request (<code>timems</code>).</p>
    ///   - [`suggest(Option<SuggestModel>)`](crate::output::SuggestOutput::suggest): <p>Container for the matching search suggestion information.</p>
                            /// - On failure, responds with [`SdkError<SuggestError>`](crate::error::SuggestError)
    pub fn suggest(&self) -> crate::client::fluent_builders::Suggest {
                                crate::client::fluent_builders::Suggest::new(self.handle.clone())
                            }
}

