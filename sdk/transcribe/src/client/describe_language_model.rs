// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeLanguageModel`](crate::client::fluent_builders::DescribeLanguageModel) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`model_name(impl Into<String>)`](crate::client::fluent_builders::DescribeLanguageModel::model_name) / [`set_model_name(Option<String>)`](crate::client::fluent_builders::DescribeLanguageModel::set_model_name): <p>The name of the custom language model you want information about. Model names are case sensitive.</p>
                            /// - On success, responds with [`DescribeLanguageModelOutput`](crate::output::DescribeLanguageModelOutput) with field(s):
    ///   - [`language_model(Option<LanguageModel>)`](crate::output::DescribeLanguageModelOutput::language_model): <p>Provides information about the specified custom language model.</p>  <p>This parameter also shows if the base language model you used to create your custom language model has been updated. If Amazon Transcribe has updated the base model, you can create a new custom language model using the updated base model.</p>  <p>If you tried to create a new custom language model and the request wasn't successful, you can use this <code>DescribeLanguageModel</code> to help identify the reason for this failure.</p>
                            /// - On failure, responds with [`SdkError<DescribeLanguageModelError>`](crate::error::DescribeLanguageModelError)
    pub fn describe_language_model(&self) -> crate::client::fluent_builders::DescribeLanguageModel {
                                crate::client::fluent_builders::DescribeLanguageModel::new(self.handle.clone())
                            }
}

