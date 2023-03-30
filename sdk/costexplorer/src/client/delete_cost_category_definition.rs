// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteCostCategoryDefinition`](crate::client::fluent_builders::DeleteCostCategoryDefinition) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`cost_category_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteCostCategoryDefinition::cost_category_arn) / [`set_cost_category_arn(Option<String>)`](crate::client::fluent_builders::DeleteCostCategoryDefinition::set_cost_category_arn): <p>The unique identifier for your Cost Category. </p>
                            /// - On success, responds with [`DeleteCostCategoryDefinitionOutput`](crate::output::DeleteCostCategoryDefinitionOutput) with field(s):
    ///   - [`cost_category_arn(Option<String>)`](crate::output::DeleteCostCategoryDefinitionOutput::cost_category_arn): <p>The unique identifier for your Cost Category. </p>
    ///   - [`effective_end(Option<String>)`](crate::output::DeleteCostCategoryDefinitionOutput::effective_end): <p>The effective end date of the Cost Category as a result of deleting it. No costs after this date is categorized by the deleted Cost Category. </p>
                            /// - On failure, responds with [`SdkError<DeleteCostCategoryDefinitionError>`](crate::error::DeleteCostCategoryDefinitionError)
    pub fn delete_cost_category_definition(&self) -> crate::client::fluent_builders::DeleteCostCategoryDefinition {
                                crate::client::fluent_builders::DeleteCostCategoryDefinition::new(self.handle.clone())
                            }
}

