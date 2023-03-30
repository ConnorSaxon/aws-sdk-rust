// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateCapacityProvider`](crate::client::fluent_builders::CreateCapacityProvider) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateCapacityProvider::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateCapacityProvider::set_name): <p>The name of the capacity provider. Up to 255 characters are allowed. They include letters (both upper and lowercase letters), numbers, underscores (_), and hyphens (-). The name can't be prefixed with "<code>aws</code>", "<code>ecs</code>", or "<code>fargate</code>".</p>
    ///   - [`auto_scaling_group_provider(AutoScalingGroupProvider)`](crate::client::fluent_builders::CreateCapacityProvider::auto_scaling_group_provider) / [`set_auto_scaling_group_provider(Option<AutoScalingGroupProvider>)`](crate::client::fluent_builders::CreateCapacityProvider::set_auto_scaling_group_provider): <p>The details of the Auto Scaling group for the capacity provider.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateCapacityProvider::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateCapacityProvider::set_tags): <p>The metadata that you apply to the capacity provider to categorize and organize them more conveniently. Each tag consists of a key and an optional value. You define both of them.</p>  <p>The following basic restrictions apply to tags:</p>  <ul>   <li> <p>Maximum number of tags per resource - 50</p> </li>   <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li>   <li> <p>Maximum key length - 128 Unicode characters in UTF-8</p> </li>   <li> <p>Maximum value length - 256 Unicode characters in UTF-8</p> </li>   <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li>   <li> <p>Tag keys and values are case-sensitive.</p> </li>   <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for either keys or values as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys or values with this prefix. Tags with this prefix do not count against your tags per resource limit.</p> </li>  </ul>
                            /// - On success, responds with [`CreateCapacityProviderOutput`](crate::output::CreateCapacityProviderOutput) with field(s):
    ///   - [`capacity_provider(Option<CapacityProvider>)`](crate::output::CreateCapacityProviderOutput::capacity_provider): <p>The full description of the new capacity provider.</p>
                            /// - On failure, responds with [`SdkError<CreateCapacityProviderError>`](crate::error::CreateCapacityProviderError)
    pub fn create_capacity_provider(&self) -> crate::client::fluent_builders::CreateCapacityProvider {
                                crate::client::fluent_builders::CreateCapacityProvider::new(self.handle.clone())
                            }
}

