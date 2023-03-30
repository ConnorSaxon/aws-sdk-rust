// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteImageRecipe`](crate::client::fluent_builders::DeleteImageRecipe) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`image_recipe_arn(impl Into<String>)`](crate::client::fluent_builders::DeleteImageRecipe::image_recipe_arn) / [`set_image_recipe_arn(Option<String>)`](crate::client::fluent_builders::DeleteImageRecipe::set_image_recipe_arn): <p>The Amazon Resource Name (ARN) of the image recipe to delete.</p>
                            /// - On success, responds with [`DeleteImageRecipeOutput`](crate::output::DeleteImageRecipeOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::output::DeleteImageRecipeOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`image_recipe_arn(Option<String>)`](crate::output::DeleteImageRecipeOutput::image_recipe_arn): <p>The Amazon Resource Name (ARN) of the image recipe that was deleted.</p>
                            /// - On failure, responds with [`SdkError<DeleteImageRecipeError>`](crate::error::DeleteImageRecipeError)
    pub fn delete_image_recipe(&self) -> crate::client::fluent_builders::DeleteImageRecipe {
                                crate::client::fluent_builders::DeleteImageRecipe::new(self.handle.clone())
                            }
}

