// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetContainerRecipe`](crate::client::fluent_builders::GetContainerRecipe) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`container_recipe_arn(impl Into<String>)`](crate::client::fluent_builders::GetContainerRecipe::container_recipe_arn) / [`set_container_recipe_arn(Option<String>)`](crate::client::fluent_builders::GetContainerRecipe::set_container_recipe_arn): <p>The Amazon Resource Name (ARN) of the container recipe to retrieve.</p>
                            /// - On success, responds with [`GetContainerRecipeOutput`](crate::output::GetContainerRecipeOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::output::GetContainerRecipeOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`container_recipe(Option<ContainerRecipe>)`](crate::output::GetContainerRecipeOutput::container_recipe): <p>The container recipe object that is returned.</p>
                            /// - On failure, responds with [`SdkError<GetContainerRecipeError>`](crate::error::GetContainerRecipeError)
    pub fn get_container_recipe(&self) -> crate::client::fluent_builders::GetContainerRecipe {
                                crate::client::fluent_builders::GetContainerRecipe::new(self.handle.clone())
                            }
}

