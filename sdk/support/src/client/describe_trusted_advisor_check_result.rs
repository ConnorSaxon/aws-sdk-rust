// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeTrustedAdvisorCheckResult`](crate::client::fluent_builders::DescribeTrustedAdvisorCheckResult) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`check_id(impl Into<String>)`](crate::client::fluent_builders::DescribeTrustedAdvisorCheckResult::check_id) / [`set_check_id(Option<String>)`](crate::client::fluent_builders::DescribeTrustedAdvisorCheckResult::set_check_id): <p>The unique identifier for the Trusted Advisor check.</p>
    ///   - [`language(impl Into<String>)`](crate::client::fluent_builders::DescribeTrustedAdvisorCheckResult::language) / [`set_language(Option<String>)`](crate::client::fluent_builders::DescribeTrustedAdvisorCheckResult::set_language): <p>The ISO 639-1 code for the language that you want your check results to appear in.</p>  <p>The Amazon Web Services Support API currently supports the following languages for Trusted Advisor:</p>  <ul>   <li> <p>Chinese, Simplified - <code>zh</code> </p> </li>   <li> <p>Chinese, Traditional - <code>zh_TW</code> </p> </li>   <li> <p>English - <code>en</code> </p> </li>   <li> <p>French - <code>fr</code> </p> </li>   <li> <p>German - <code>de</code> </p> </li>   <li> <p>Indonesian - <code>id</code> </p> </li>   <li> <p>Italian - <code>it</code> </p> </li>   <li> <p>Japanese - <code>ja</code> </p> </li>   <li> <p>Korean - <code>ko</code> </p> </li>   <li> <p>Portuguese, Brazilian - <code>pt_BR</code> </p> </li>   <li> <p>Spanish - <code>es</code> </p> </li>  </ul>
                            /// - On success, responds with [`DescribeTrustedAdvisorCheckResultOutput`](crate::output::DescribeTrustedAdvisorCheckResultOutput) with field(s):
    ///   - [`result(Option<TrustedAdvisorCheckResult>)`](crate::output::DescribeTrustedAdvisorCheckResultOutput::result): <p>The detailed results of the Trusted Advisor check.</p>
                            /// - On failure, responds with [`SdkError<DescribeTrustedAdvisorCheckResultError>`](crate::error::DescribeTrustedAdvisorCheckResultError)
    pub fn describe_trusted_advisor_check_result(&self) -> crate::client::fluent_builders::DescribeTrustedAdvisorCheckResult {
                                crate::client::fluent_builders::DescribeTrustedAdvisorCheckResult::new(self.handle.clone())
                            }
}

