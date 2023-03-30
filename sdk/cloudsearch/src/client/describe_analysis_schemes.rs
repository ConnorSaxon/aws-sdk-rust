// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeAnalysisSchemes`](crate::client::fluent_builders::DescribeAnalysisSchemes) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::client::fluent_builders::DescribeAnalysisSchemes::domain_name) / [`set_domain_name(Option<String>)`](crate::client::fluent_builders::DescribeAnalysisSchemes::set_domain_name): <p>The name of the domain you want to describe.</p>
    ///   - [`analysis_scheme_names(Vec<String>)`](crate::client::fluent_builders::DescribeAnalysisSchemes::analysis_scheme_names) / [`set_analysis_scheme_names(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeAnalysisSchemes::set_analysis_scheme_names): <p>The analysis schemes you want to describe.</p>
    ///   - [`deployed(bool)`](crate::client::fluent_builders::DescribeAnalysisSchemes::deployed) / [`set_deployed(Option<bool>)`](crate::client::fluent_builders::DescribeAnalysisSchemes::set_deployed): <p>Whether to display the deployed configuration (<code>true</code>) or include any pending changes (<code>false</code>). Defaults to <code>false</code>.</p>
                            /// - On success, responds with [`DescribeAnalysisSchemesOutput`](crate::output::DescribeAnalysisSchemesOutput) with field(s):
    ///   - [`analysis_schemes(Option<Vec<AnalysisSchemeStatus>>)`](crate::output::DescribeAnalysisSchemesOutput::analysis_schemes): <p>The analysis scheme descriptions.</p>
                            /// - On failure, responds with [`SdkError<DescribeAnalysisSchemesError>`](crate::error::DescribeAnalysisSchemesError)
    pub fn describe_analysis_schemes(&self) -> crate::client::fluent_builders::DescribeAnalysisSchemes {
                                crate::client::fluent_builders::DescribeAnalysisSchemes::new(self.handle.clone())
                            }
}

