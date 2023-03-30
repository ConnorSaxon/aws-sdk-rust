// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListFunctions`](crate::client::fluent_builders::ListFunctions) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListFunctions::into_paginator).
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`master_region(impl Into<String>)`](crate::client::fluent_builders::ListFunctions::master_region) / [`set_master_region(Option<String>)`](crate::client::fluent_builders::ListFunctions::set_master_region): <p>For Lambda@Edge functions, the Amazon Web Services Region of the master function. For example, <code>us-east-1</code> filters the list of functions to include only Lambda@Edge functions replicated from a master function in US East (N. Virginia). If specified, you must set <code>FunctionVersion</code> to <code>ALL</code>.</p>
    ///   - [`function_version(FunctionVersion)`](crate::client::fluent_builders::ListFunctions::function_version) / [`set_function_version(Option<FunctionVersion>)`](crate::client::fluent_builders::ListFunctions::set_function_version): <p>Set to <code>ALL</code> to include entries for all published versions of each function.</p>
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListFunctions::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListFunctions::set_marker): <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    ///   - [`max_items(i32)`](crate::client::fluent_builders::ListFunctions::max_items) / [`set_max_items(Option<i32>)`](crate::client::fluent_builders::ListFunctions::set_max_items): <p>The maximum number of functions to return in the response. Note that <code>ListFunctions</code> returns a maximum of 50 items in each response, even if you set the number higher.</p>
                            /// - On success, responds with [`ListFunctionsOutput`](crate::output::ListFunctionsOutput) with field(s):
    ///   - [`next_marker(Option<String>)`](crate::output::ListFunctionsOutput::next_marker): <p>The pagination token that's included if more results are available.</p>
    ///   - [`functions(Option<Vec<FunctionConfiguration>>)`](crate::output::ListFunctionsOutput::functions): <p>A list of Lambda functions.</p>
                            /// - On failure, responds with [`SdkError<ListFunctionsError>`](crate::error::ListFunctionsError)
    pub fn list_functions(&self) -> crate::client::fluent_builders::ListFunctions {
                                crate::client::fluent_builders::ListFunctions::new(self.handle.clone())
                            }
}

