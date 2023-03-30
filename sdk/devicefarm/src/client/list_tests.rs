// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTests`](crate::client::fluent_builders::ListTests) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListTests::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::client::fluent_builders::ListTests::arn) / [`set_arn(Option<String>)`](crate::client::fluent_builders::ListTests::set_arn): <p>The test suite's Amazon Resource Name (ARN).</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListTests::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListTests::set_next_token): <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
                            /// - On success, responds with [`ListTestsOutput`](crate::output::ListTestsOutput) with field(s):
    ///   - [`tests(Option<Vec<Test>>)`](crate::output::ListTestsOutput::tests): <p>Information about the tests.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListTestsOutput::next_token): <p>If the number of items that are returned is significantly large, this is an identifier that is also returned. It can be used in a subsequent call to this operation to return the next set of items in the list.</p>
                            /// - On failure, responds with [`SdkError<ListTestsError>`](crate::error::ListTestsError)
    pub fn list_tests(&self) -> crate::client::fluent_builders::ListTests {
                                crate::client::fluent_builders::ListTests::new(self.handle.clone())
                            }
}

