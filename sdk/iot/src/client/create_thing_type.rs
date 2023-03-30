// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateThingType`](crate::client::fluent_builders::CreateThingType) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`thing_type_name(impl Into<String>)`](crate::client::fluent_builders::CreateThingType::thing_type_name) / [`set_thing_type_name(Option<String>)`](crate::client::fluent_builders::CreateThingType::set_thing_type_name): <p>The name of the thing type.</p>
    ///   - [`thing_type_properties(ThingTypeProperties)`](crate::client::fluent_builders::CreateThingType::thing_type_properties) / [`set_thing_type_properties(Option<ThingTypeProperties>)`](crate::client::fluent_builders::CreateThingType::set_thing_type_properties): <p>The ThingTypeProperties for the thing type to create. It contains information about the new thing type including a description, and a list of searchable thing attribute names.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateThingType::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateThingType::set_tags): <p>Metadata which can be used to manage the thing type.</p>
                            /// - On success, responds with [`CreateThingTypeOutput`](crate::output::CreateThingTypeOutput) with field(s):
    ///   - [`thing_type_name(Option<String>)`](crate::output::CreateThingTypeOutput::thing_type_name): <p>The name of the thing type.</p>
    ///   - [`thing_type_arn(Option<String>)`](crate::output::CreateThingTypeOutput::thing_type_arn): <p>The Amazon Resource Name (ARN) of the thing type.</p>
    ///   - [`thing_type_id(Option<String>)`](crate::output::CreateThingTypeOutput::thing_type_id): <p>The thing type ID.</p>
                            /// - On failure, responds with [`SdkError<CreateThingTypeError>`](crate::error::CreateThingTypeError)
    pub fn create_thing_type(&self) -> crate::client::fluent_builders::CreateThingType {
                                crate::client::fluent_builders::CreateThingType::new(self.handle.clone())
                            }
}

