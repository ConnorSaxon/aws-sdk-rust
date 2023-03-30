// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetAnalyzer`](crate::client::fluent_builders::GetAnalyzer) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`analyzer_name(impl Into<String>)`](crate::client::fluent_builders::GetAnalyzer::analyzer_name) / [`set_analyzer_name(Option<String>)`](crate::client::fluent_builders::GetAnalyzer::set_analyzer_name): <p>The name of the analyzer retrieved.</p>
                            /// - On success, responds with [`GetAnalyzerOutput`](crate::output::GetAnalyzerOutput) with field(s):
    ///   - [`analyzer(Option<AnalyzerSummary>)`](crate::output::GetAnalyzerOutput::analyzer): <p>An <code>AnalyzerSummary</code> object that contains information about the analyzer.</p>
                            /// - On failure, responds with [`SdkError<GetAnalyzerError>`](crate::error::GetAnalyzerError)
    pub fn get_analyzer(&self) -> crate::client::fluent_builders::GetAnalyzer {
                                crate::client::fluent_builders::GetAnalyzer::new(self.handle.clone())
                            }
}

