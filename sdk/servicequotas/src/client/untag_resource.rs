// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UntagResource`](crate::client::fluent_builders::UntagResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::client::fluent_builders::UntagResource::resource_arn) / [`set_resource_arn(Option<String>)`](crate::client::fluent_builders::UntagResource::set_resource_arn): <p>The Amazon Resource Name (ARN) for the applied quota that you want to untag. You can get this information by using the Service Quotas console, or by listing the quotas using the <a href="https://docs.aws.amazon.com/cli/latest/reference/service-quotas/list-service-quotas.html">list-service-quotas</a> AWS CLI command or the <a href="https://docs.aws.amazon.com/servicequotas/2019-06-24/apireference/API_ListServiceQuotas.html">ListServiceQuotas</a> AWS API operation.</p>
    ///   - [`tag_keys(Vec<String>)`](crate::client::fluent_builders::UntagResource::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::client::fluent_builders::UntagResource::set_tag_keys): <p>The keys of the tags that you want to remove from the resource.</p>
                            /// - On success, responds with [`UntagResourceOutput`](crate::output::UntagResourceOutput)
                            /// - On failure, responds with [`SdkError<UntagResourceError>`](crate::error::UntagResourceError)
    pub fn untag_resource(&self) -> crate::client::fluent_builders::UntagResource {
                                crate::client::fluent_builders::UntagResource::new(self.handle.clone())
                            }
}

