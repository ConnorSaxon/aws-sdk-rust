// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListUniqueProblems`](crate::client::fluent_builders::ListUniqueProblems) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListUniqueProblems::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::ListUniqueProblems::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::ListUniqueProblems::set_arn): <p>The unique problems' ARNs.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListUniqueProblems::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListUniqueProblems::set_next_token): <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
                            /// - On success, responds with [`ListUniqueProblemsOutput`](crate::output::ListUniqueProblemsOutput) with field(s):
    ///   - [`unique_problems(Option<HashMap<ExecutionResult, Vec<UniqueProblem>>>)`](crate::output::ListUniqueProblemsOutput::unique_problems): <p>Information about the unique problems.</p>  <p>Allowed values include:</p>  <ul>   <li> <p>PENDING</p> </li>   <li> <p>PASSED</p> </li>   <li> <p>WARNED</p> </li>   <li> <p>FAILED</p> </li>   <li> <p>SKIPPED</p> </li>   <li> <p>ERRORED</p> </li>   <li> <p>STOPPED</p> </li>  </ul>
    ///   - [`next_token(Option<String>)`](crate::output::ListUniqueProblemsOutput::next_token): <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
                            /// - On failure, responds with [`SdkError<ListUniqueProblemsError>`](crate::error::ListUniqueProblemsError)
    pub fn list_unique_problems(&self) -> crate::client::fluent_builders::ListUniqueProblems {
                                crate::client::fluent_builders::ListUniqueProblems::new(self.handle.clone())
                            }
}

