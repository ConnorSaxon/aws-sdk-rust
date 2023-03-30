// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateUserDefinedFunction`](crate::client::fluent_builders::UpdateUserDefinedFunction) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::client::fluent_builders::UpdateUserDefinedFunction::catalog_id) / [`set_catalog_id(Option<String>)`](crate::client::fluent_builders::UpdateUserDefinedFunction::set_catalog_id): <p>The ID of the Data Catalog where the function to be updated is located. If none is provided, the Amazon Web Services account ID is used by default.</p>
    ///   - [`database_name(impl Into<String>)`](crate::client::fluent_builders::UpdateUserDefinedFunction::database_name) / [`set_database_name(Option<String>)`](crate::client::fluent_builders::UpdateUserDefinedFunction::set_database_name): <p>The name of the catalog database where the function to be updated is located.</p>
    ///   - [`function_name(impl Into<String>)`](crate::client::fluent_builders::UpdateUserDefinedFunction::function_name) / [`set_function_name(Option<String>)`](crate::client::fluent_builders::UpdateUserDefinedFunction::set_function_name): <p>The name of the function.</p>
    ///   - [`function_input(UserDefinedFunctionInput)`](crate::client::fluent_builders::UpdateUserDefinedFunction::function_input) / [`set_function_input(Option<UserDefinedFunctionInput>)`](crate::client::fluent_builders::UpdateUserDefinedFunction::set_function_input): <p>A <code>FunctionInput</code> object that redefines the function in the Data Catalog.</p>
                            /// - On success, responds with [`UpdateUserDefinedFunctionOutput`](crate::output::UpdateUserDefinedFunctionOutput)
                            /// - On failure, responds with [`SdkError<UpdateUserDefinedFunctionError>`](crate::error::UpdateUserDefinedFunctionError)
    pub fn update_user_defined_function(&self) -> crate::client::fluent_builders::UpdateUserDefinedFunction {
                                crate::client::fluent_builders::UpdateUserDefinedFunction::new(self.handle.clone())
                            }
}

