// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AssociateLambdaFunction`](crate::client::fluent_builders::AssociateLambdaFunction) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::AssociateLambdaFunction::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::AssociateLambdaFunction::set_instance_id): <p>The identifier of the Amazon Connect instance. You can find the instanceId in the ARN of the instance.</p>
    ///   - [`function_arn(impl Into<String>)`](crate::client::fluent_builders::AssociateLambdaFunction::function_arn) / [`set_function_arn(Option<String>)`](crate::client::fluent_builders::AssociateLambdaFunction::set_function_arn): <p>The Amazon Resource Name (ARN) for the Lambda function being associated. Maximum number of characters allowed is 140.</p>
                            /// - On success, responds with [`AssociateLambdaFunctionOutput`](crate::output::AssociateLambdaFunctionOutput)
                            /// - On failure, responds with [`SdkError<AssociateLambdaFunctionError>`](crate::error::AssociateLambdaFunctionError)
    pub fn associate_lambda_function(&self) -> crate::client::fluent_builders::AssociateLambdaFunction {
                                crate::client::fluent_builders::AssociateLambdaFunction::new(self.handle.clone())
                            }
}

