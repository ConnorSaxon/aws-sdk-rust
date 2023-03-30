// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListVersionsByFunction`](crate::client::fluent_builders::ListVersionsByFunction) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListVersionsByFunction::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`function_name(impl Into<String>)`](crate::client::fluent_builders::ListVersionsByFunction::function_name) / [`set_function_name(Option<String>)`](crate::client::fluent_builders::ListVersionsByFunction::set_function_name): <p>The name of the Lambda function.</p>  <p class="title"> <b>Name formats</b> </p>  <ul>   <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li>   <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li>   <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li>  </ul>  <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListVersionsByFunction::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListVersionsByFunction::set_marker): <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    ///   - [`max_items(i32)`](crate::client::fluent_builders::ListVersionsByFunction::max_items) / [`set_max_items(Option<i32>)`](crate::client::fluent_builders::ListVersionsByFunction::set_max_items): <p>The maximum number of versions to return. Note that <code>ListVersionsByFunction</code> returns a maximum of 50 items in each response, even if you set the number higher.</p>
                            /// - On success, responds with [`ListVersionsByFunctionOutput`](crate::output::ListVersionsByFunctionOutput) with field(s):
    ///   - [`next_marker(Option<String>)`](crate::output::ListVersionsByFunctionOutput::next_marker): <p>The pagination token that's included if more results are available.</p>
    ///   - [`versions(Option<Vec<FunctionConfiguration>>)`](crate::output::ListVersionsByFunctionOutput::versions): <p>A list of Lambda function versions.</p>
                            /// - On failure, responds with [`SdkError<ListVersionsByFunctionError>`](crate::error::ListVersionsByFunctionError)
    pub fn list_versions_by_function(&self) -> crate::client::fluent_builders::ListVersionsByFunction {
                                crate::client::fluent_builders::ListVersionsByFunction::new(self.handle.clone())
                            }
}

