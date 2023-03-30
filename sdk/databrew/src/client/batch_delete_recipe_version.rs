// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`BatchDeleteRecipeVersion`](crate::client::fluent_builders::BatchDeleteRecipeVersion) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::BatchDeleteRecipeVersion::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::BatchDeleteRecipeVersion::set_name): <p>The name of the recipe whose versions are to be deleted.</p>
    ///   - [`recipe_versions(Vec<String>)`](crate::client::fluent_builders::BatchDeleteRecipeVersion::recipe_versions) / [`set_recipe_versions(Option<Vec<String>>)`](crate::client::fluent_builders::BatchDeleteRecipeVersion::set_recipe_versions): <p>An array of version identifiers, for the recipe versions to be deleted. You can specify numeric versions (<code>X.Y</code>) or <code>LATEST_WORKING</code>. <code>LATEST_PUBLISHED</code> is not supported.</p>
                            /// - On success, responds with [`BatchDeleteRecipeVersionOutput`](crate::output::BatchDeleteRecipeVersionOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::BatchDeleteRecipeVersionOutput::name): <p>The name of the recipe that was modified.</p>
    ///   - [`errors(Option<Vec<RecipeVersionErrorDetail>>)`](crate::output::BatchDeleteRecipeVersionOutput::errors): <p>Errors, if any, that occurred while attempting to delete the recipe versions.</p>
                            /// - On failure, responds with [`SdkError<BatchDeleteRecipeVersionError>`](crate::error::BatchDeleteRecipeVersionError)
    pub fn batch_delete_recipe_version(&self) -> crate::client::fluent_builders::BatchDeleteRecipeVersion {
                                crate::client::fluent_builders::BatchDeleteRecipeVersion::new(self.handle.clone())
                            }
}

