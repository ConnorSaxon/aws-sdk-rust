// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeReleaseLabel`](crate::client::fluent_builders::DescribeReleaseLabel) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`release_label(impl Into<String>)`](crate::client::fluent_builders::DescribeReleaseLabel::release_label) / [`set_release_label(Option<String>)`](crate::client::fluent_builders::DescribeReleaseLabel::set_release_label): <p>The target release label to be described.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeReleaseLabel::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeReleaseLabel::set_next_token): <p>The pagination token. Reserved for future use. Currently set to null.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeReleaseLabel::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeReleaseLabel::set_max_results): <p>Reserved for future use. Currently set to null.</p>
                            /// - On success, responds with [`DescribeReleaseLabelOutput`](crate::output::DescribeReleaseLabelOutput) with field(s):
    ///   - [`release_label(Option<String>)`](crate::output::DescribeReleaseLabelOutput::release_label): <p>The target release label described in the response.</p>
    ///   - [`applications(Option<Vec<SimplifiedApplication>>)`](crate::output::DescribeReleaseLabelOutput::applications): <p>The list of applications available for the target release label. <code>Name</code> is the name of the application. <code>Version</code> is the concise version of the application.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeReleaseLabelOutput::next_token): <p>The pagination token. Reserved for future use. Currently set to null.</p>
    ///   - [`available_os_releases(Option<Vec<OsRelease>>)`](crate::output::DescribeReleaseLabelOutput::available_os_releases): <p>The list of available Amazon Linux release versions for an Amazon EMR release. Contains a Label field that is formatted as shown in <a href="https://docs.aws.amazon.com/AL2/latest/relnotes/relnotes-al2.html"> <i>Amazon Linux 2 Release Notes</i> </a>. For example, <a href="https://docs.aws.amazon.com/AL2/latest/relnotes/relnotes-20220218.html">2.0.20220218.1</a>.</p>
                            /// - On failure, responds with [`SdkError<DescribeReleaseLabelError>`](crate::error::DescribeReleaseLabelError)
    pub fn describe_release_label(&self) -> crate::client::fluent_builders::DescribeReleaseLabel {
                                crate::client::fluent_builders::DescribeReleaseLabel::new(self.handle.clone())
                            }
}

