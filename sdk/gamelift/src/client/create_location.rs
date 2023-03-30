// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateLocation`](crate::client::fluent_builders::CreateLocation) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`location_name(impl Into<String>)`](crate::client::fluent_builders::CreateLocation::location_name) / [`set_location_name(Option<String>)`](crate::client::fluent_builders::CreateLocation::set_location_name): <p>A descriptive name for the custom location.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateLocation::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateLocation::set_tags): <p>A list of labels to assign to the new matchmaking configuration resource. Tags are developer-defined key-value pairs. Tagging Amazon Web Services resources are useful for resource management, access management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging Amazon Web Services Resources</a> in the <i>Amazon Web Services General Rareference</i>.</p>
                            /// - On success, responds with [`CreateLocationOutput`](crate::output::CreateLocationOutput) with field(s):
    ///   - [`location(Option<LocationModel>)`](crate::output::CreateLocationOutput::location): <p>The details of the custom location you created.</p>
                            /// - On failure, responds with [`SdkError<CreateLocationError>`](crate::error::CreateLocationError)
    pub fn create_location(&self) -> crate::client::fluent_builders::CreateLocation {
                                crate::client::fluent_builders::CreateLocation::new(self.handle.clone())
                            }
}

