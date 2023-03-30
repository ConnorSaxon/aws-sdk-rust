// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateAlias`](crate::client::fluent_builders::CreateAlias) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateAlias::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateAlias::set_name): <p>A descriptive label that is associated with an alias. Alias names do not need to be unique.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateAlias::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateAlias::set_description): <p>A human-readable description of the alias.</p>
    ///   - [`routing_strategy(RoutingStrategy)`](crate::client::fluent_builders::CreateAlias::routing_strategy) / [`set_routing_strategy(Option<RoutingStrategy>)`](crate::client::fluent_builders::CreateAlias::set_routing_strategy): <p>The routing configuration, including routing type and fleet target, for the alias. </p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateAlias::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateAlias::set_tags): <p>A list of labels to assign to the new alias resource. Tags are developer-defined key-value pairs. Tagging Amazon Web Services resources are useful for resource management, access management and cost allocation. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html"> Tagging Amazon Web Services Resources</a> in the <i>Amazon Web Services General Reference</i>.</p>
                            /// - On success, responds with [`CreateAliasOutput`](crate::output::CreateAliasOutput) with field(s):
    ///   - [`alias(Option<Alias>)`](crate::output::CreateAliasOutput::alias): <p>The newly created alias resource.</p>
                            /// - On failure, responds with [`SdkError<CreateAliasError>`](crate::error::CreateAliasError)
    pub fn create_alias(&self) -> crate::client::fluent_builders::CreateAlias {
                                crate::client::fluent_builders::CreateAlias::new(self.handle.clone())
                            }
}

