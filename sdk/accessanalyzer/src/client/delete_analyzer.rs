// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteAnalyzer`](crate::client::fluent_builders::DeleteAnalyzer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`analyzer_name(impl Into<String>)`](crate::client::fluent_builders::DeleteAnalyzer::analyzer_name) / [`set_analyzer_name(Option<String>)`](crate::client::fluent_builders::DeleteAnalyzer::set_analyzer_name): <p>The name of the analyzer to delete.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::DeleteAnalyzer::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::DeleteAnalyzer::set_client_token): <p>A client token.</p>
                            /// - On success, responds with [`DeleteAnalyzerOutput`](crate::output::DeleteAnalyzerOutput)
                            /// - On failure, responds with [`SdkError<DeleteAnalyzerError>`](crate::error::DeleteAnalyzerError)
    pub fn delete_analyzer(&self) -> crate::client::fluent_builders::DeleteAnalyzer {
                                crate::client::fluent_builders::DeleteAnalyzer::new(self.handle.clone())
                            }
}

