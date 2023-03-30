// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListFunctions`](crate::client::fluent_builders::ListFunctions) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`marker(impl Into<String>)`](crate::client::fluent_builders::ListFunctions::marker) / [`set_marker(Option<String>)`](crate::client::fluent_builders::ListFunctions::set_marker): <p>Use this field when paginating results to indicate where to begin in your list of functions. The response includes functions in the list that occur after the marker. To get the next page of the list, set this field's value to the value of <code>NextMarker</code> from the current page's response.</p>
    ///   - [`max_items(i32)`](crate::client::fluent_builders::ListFunctions::max_items) / [`set_max_items(Option<i32>)`](crate::client::fluent_builders::ListFunctions::set_max_items): <p>The maximum number of functions that you want in the response.</p>
    ///   - [`stage(FunctionStage)`](crate::client::fluent_builders::ListFunctions::stage) / [`set_stage(Option<FunctionStage>)`](crate::client::fluent_builders::ListFunctions::set_stage): <p>An optional filter to return only the functions that are in the specified stage, either <code>DEVELOPMENT</code> or <code>LIVE</code>.</p>
                            /// - On success, responds with [`ListFunctionsOutput`](crate::output::ListFunctionsOutput) with field(s):
    ///   - [`function_list(Option<FunctionList>)`](crate::output::ListFunctionsOutput::function_list): <p>A list of CloudFront functions.</p>
                            /// - On failure, responds with [`SdkError<ListFunctionsError>`](crate::error::ListFunctionsError)
    pub fn list_functions(&self) -> crate::client::fluent_builders::ListFunctions {
                                crate::client::fluent_builders::ListFunctions::new(self.handle.clone())
                            }
}

