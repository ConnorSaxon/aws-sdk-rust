// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::client::fluent_builders::ListTagsForResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::ListTagsForResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::ListTagsForResource::set_resource_arn): <p>The Amazon Resource Name (ARN) for the applied quota for which you want to list tags. You can get this information by using the Service Quotas console, or by listing the quotas using the <a href="https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html">list-service-quotas</a> AWS CLI command or the <a href="https://docs.aws.amazon.com/servicequotas/2019-06-24/apireference/API_ListServiceQuotas.html">ListServiceQuotas</a> AWS API operation.</p>
                            /// - On success, responds with [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput) with field(s):
    ///   - [`tags(Option<Vec<Tag>>)`](crate::output::ListTagsForResourceOutput::tags): <p>A complex data type that contains zero or more tag elements.</p>
                            /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::error::ListTagsForResourceError)
    pub fn list_tags_for_resource(&self) -> crate::client::fluent_builders::ListTagsForResource {
                                crate::client::fluent_builders::ListTagsForResource::new(self.handle.clone())
                            }
}

