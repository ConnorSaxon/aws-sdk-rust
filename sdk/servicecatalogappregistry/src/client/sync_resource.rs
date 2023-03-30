// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`SyncResource`](crate::client::fluent_builders::SyncResource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`resource_type(ResourceType)`](crate::client::fluent_builders::SyncResource::resource_type) / [`set_resource_type(Option<ResourceType>)`](crate::client::fluent_builders::SyncResource::set_resource_type): <p>The type of resource of which the application will be associated.</p>
    ///   - [`resource(impl Into<String>)`](crate::client::fluent_builders::SyncResource::resource) / [`set_resource(Option<String>)`](crate::client::fluent_builders::SyncResource::set_resource): <p>An entity you can work with and specify with a name or ID. Examples include an Amazon EC2 instance, an Amazon Web Services CloudFormation stack, or an Amazon S3 bucket.</p>
                            /// - On success, responds with [`SyncResourceOutput`](crate::output::SyncResourceOutput) with field(s):
    ///   - [`application_arn(Option<String>)`](crate::output::SyncResourceOutput::application_arn): <p>The Amazon resource name (ARN) that specifies the application.</p>
    ///   - [`resource_arn(Option<String>)`](crate::output::SyncResourceOutput::resource_arn): <p>The Amazon resource name (ARN) that specifies the resource.</p>
    ///   - [`action_taken(Option<SyncAction>)`](crate::output::SyncResourceOutput::action_taken): <p>The results of the output if an application is associated with an ARN value, which could be <code>syncStarted</code> or None.</p>
                            /// - On failure, responds with [`SdkError<SyncResourceError>`](crate::error::SyncResourceError)
    pub fn sync_resource(&self) -> crate::client::fluent_builders::SyncResource {
                                crate::client::fluent_builders::SyncResource::new(self.handle.clone())
                            }
}

