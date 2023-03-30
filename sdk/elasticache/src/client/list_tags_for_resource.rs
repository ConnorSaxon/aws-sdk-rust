// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::client::fluent_builders::ListTagsForResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_name(impl Into<String>)`](crate::client::fluent_builders::ListTagsForResource::resource_name) / [`set_resource_name(Option<String>)`](crate::client::fluent_builders::ListTagsForResource::set_resource_name): <p>The Amazon Resource Name (ARN) of the resource for which you want the list of tags, for example <code>arn:aws:elasticache:us-west-2:0123456789:cluster:myCluster</code> or <code>arn:aws:elasticache:us-west-2:0123456789:snapshot:mySnapshot</code>.</p>  <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p>
                            /// - On success, responds with [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput) with field(s):
    ///   - [`tag_list(Option<Vec<Tag>>)`](crate::output::ListTagsForResourceOutput::tag_list): <p>A list of tags as key-value pairs.</p>
                            /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::error::ListTagsForResourceError)
    pub fn list_tags_for_resource(&self) -> crate::client::fluent_builders::ListTagsForResource {
                                crate::client::fluent_builders::ListTagsForResource::new(self.handle.clone())
                            }
}

