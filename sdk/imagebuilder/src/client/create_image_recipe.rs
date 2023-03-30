// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateImageRecipe`](crate::client::fluent_builders::CreateImageRecipe) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateImageRecipe::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateImageRecipe::set_name): <p> The name of the image recipe.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateImageRecipe::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateImageRecipe::set_description): <p> The description of the image recipe.</p>
    ///   - [`semantic_version(impl Into<String>)`](crate::client::fluent_builders::CreateImageRecipe::semantic_version) / [`set_semantic_version(Option<String>)`](crate::client::fluent_builders::CreateImageRecipe::set_semantic_version): <p>The semantic version of the image recipe. This version follows the semantic version syntax.</p> <note>   <p>The semantic version has four nodes: <major>    .    <minor>     .     <patch>      /      <build>       . You can assign values for the first three, and can filter on all of them.      </build>     </patch>    </minor>   </major></p>   <p> <b>Assignment:</b> For the first three nodes you can assign any positive integer value, including zero, with an upper limit of 2^30-1, or 1073741823 for each node. Image Builder automatically assigns the build number to the fourth node.</p>   <p> <b>Patterns:</b> You can use any numeric pattern that adheres to the assignment requirements for the nodes that you can assign. For example, you might choose a software version pattern, such as 1.0.0, or a date, such as 2021.01.01.</p>  </note>
    ///   - [`components(Vec<ComponentConfiguration>)`](crate::client::fluent_builders::CreateImageRecipe::components) / [`set_components(Option<Vec<ComponentConfiguration>>)`](crate::client::fluent_builders::CreateImageRecipe::set_components): <p>The components included in the image recipe.</p>
    ///   - [`parent_image(impl Into<String>)`](crate::client::fluent_builders::CreateImageRecipe::parent_image) / [`set_parent_image(Option<String>)`](crate::client::fluent_builders::CreateImageRecipe::set_parent_image): <p>The base image of the image recipe. The value of the string can be the ARN of the base image or an AMI ID. The format for the ARN follows this example: <code>arn:aws:imagebuilder:us-west-2:aws:image/windows-server-2016-english-full-base-x86/x.x.x</code>. You can provide the specific version that you want to use, or you can use a wildcard in all of the fields. If you enter an AMI ID for the string value, you must have access to the AMI, and the AMI must be in the same Region in which you are using Image Builder.</p>
    ///   - [`block_device_mappings(Vec<InstanceBlockDeviceMapping>)`](crate::client::fluent_builders::CreateImageRecipe::block_device_mappings) / [`set_block_device_mappings(Option<Vec<InstanceBlockDeviceMapping>>)`](crate::client::fluent_builders::CreateImageRecipe::set_block_device_mappings): <p>The block device mappings of the image recipe.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateImageRecipe::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateImageRecipe::set_tags): <p> The tags of the image recipe.</p>
    ///   - [`working_directory(impl Into<String>)`](crate::client::fluent_builders::CreateImageRecipe::working_directory) / [`set_working_directory(Option<String>)`](crate::client::fluent_builders::CreateImageRecipe::set_working_directory): <p>The working directory used during build and test workflows.</p>
    ///   - [`additional_instance_configuration(AdditionalInstanceConfiguration)`](crate::client::fluent_builders::CreateImageRecipe::additional_instance_configuration) / [`set_additional_instance_configuration(Option<AdditionalInstanceConfiguration>)`](crate::client::fluent_builders::CreateImageRecipe::set_additional_instance_configuration): <p>Specify additional settings and launch scripts for your build instances.</p>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateImageRecipe::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateImageRecipe::set_client_token): <p>The idempotency token used to make this request idempotent.</p>
                            /// - On success, responds with [`CreateImageRecipeOutput`](crate::output::CreateImageRecipeOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::output::CreateImageRecipeOutput::request_id): <p>The request ID that uniquely identifies this request.</p>
    ///   - [`client_token(Option<String>)`](crate::output::CreateImageRecipeOutput::client_token): <p>The idempotency token used to make this request idempotent.</p>
    ///   - [`image_recipe_arn(Option<String>)`](crate::output::CreateImageRecipeOutput::image_recipe_arn): <p>The Amazon Resource Name (ARN) of the image recipe that was created by this request.</p>
                            /// - On failure, responds with [`SdkError<CreateImageRecipeError>`](crate::error::CreateImageRecipeError)
    pub fn create_image_recipe(&self) -> crate::client::fluent_builders::CreateImageRecipe {
                                crate::client::fluent_builders::CreateImageRecipe::new(self.handle.clone())
                            }
}

