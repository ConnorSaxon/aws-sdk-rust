// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetCustomEntityType`](crate::client::fluent_builders::GetCustomEntityType) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::GetCustomEntityType::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::GetCustomEntityType::set_name): <p>The name of the custom pattern that you want to retrieve.</p>
                            /// - On success, responds with [`GetCustomEntityTypeOutput`](crate::output::GetCustomEntityTypeOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::GetCustomEntityTypeOutput::name): <p>The name of the custom pattern that you retrieved.</p>
    ///   - [`regex_string(Option<String>)`](crate::output::GetCustomEntityTypeOutput::regex_string): <p>A regular expression string that is used for detecting sensitive data in a custom pattern.</p>
    ///   - [`context_words(Option<Vec<String>>)`](crate::output::GetCustomEntityTypeOutput::context_words): <p>A list of context words if specified when you created the custom pattern. If none of these context words are found within the vicinity of the regular expression the data will not be detected as sensitive data.</p>
                            /// - On failure, responds with [`SdkError<GetCustomEntityTypeError>`](crate::error::GetCustomEntityTypeError)
    pub fn get_custom_entity_type(&self) -> crate::client::fluent_builders::GetCustomEntityType {
                                crate::client::fluent_builders::GetCustomEntityType::new(self.handle.clone())
                            }
}

