// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdateComponent`](crate::client::fluent_builders::UpdateComponent) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdateComponent::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdateComponent::set_name): <p>The name of the component to update.</p>
    ///   - [`deployment_type(ComponentDeploymentUpdateType)`](crate::client::fluent_builders::UpdateComponent::deployment_type) / [`set_deployment_type(Option<ComponentDeploymentUpdateType>)`](crate::client::fluent_builders::UpdateComponent::set_deployment_type): <p>The deployment type. It defines the mode for updating a component, as follows:</p>  <dl>   <dt></dt>   <dd>    <p> <code>NONE</code> </p>    <p>In this mode, a deployment <i>doesn't</i> occur. Only the requested metadata parameters are updated. You can only specify <code>description</code> in this mode.</p>   </dd>   <dt></dt>   <dd>    <p> <code>CURRENT_VERSION</code> </p>    <p>In this mode, the component is deployed and updated with the new <code>serviceSpec</code>, <code>templateSource</code>, and/or <code>type</code> that you provide. Only requested parameters are updated.</p>   </dd>  </dl>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdateComponent::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdateComponent::set_description): <p>An optional customer-provided description of the component.</p>
    ///   - [`service_name(impl Into<String>)`](crate::client::fluent_builders::UpdateComponent::service_name) / [`set_service_name(Option<String>)`](crate::client::fluent_builders::UpdateComponent::set_service_name): <p>The name of the service that <code>serviceInstanceName</code> is associated with. Don't specify to keep the component's current service instance attachment. Specify an empty string to detach the component from the service instance it's attached to. Specify non-empty values for both <code>serviceInstanceName</code> and <code>serviceName</code> or for neither of them.</p>
    ///   - [`service_instance_name(impl Into<String>)`](crate::client::fluent_builders::UpdateComponent::service_instance_name) / [`set_service_instance_name(Option<String>)`](crate::client::fluent_builders::UpdateComponent::set_service_instance_name): <p>The name of the service instance that you want to attach this component to. Don't specify to keep the component's current service instance attachment. Specify an empty string to detach the component from the service instance it's attached to. Specify non-empty values for both <code>serviceInstanceName</code> and <code>serviceName</code> or for neither of them.</p>
    ///   - [`service_spec(impl Into<String>)`](crate::client::fluent_builders::UpdateComponent::service_spec) / [`set_service_spec(Option<String>)`](crate::client::fluent_builders::UpdateComponent::set_service_spec): <p>The service spec that you want the component to use to access service inputs. Set this only when the component is attached to a service instance.</p>
    ///   - [`template_file(impl Into<String>)`](crate::client::fluent_builders::UpdateComponent::template_file) / [`set_template_file(Option<String>)`](crate::client::fluent_builders::UpdateComponent::set_template_file): <p>A path to the Infrastructure as Code (IaC) file describing infrastructure that a custom component provisions.</p> <note>   <p>Components support a single IaC file, even if you use Terraform as your template language.</p>  </note>
                            /// - On success, responds with [`UpdateComponentOutput`](crate::output::UpdateComponentOutput) with field(s):
    ///   - [`component(Option<Component>)`](crate::output::UpdateComponentOutput::component): <p>The detailed data of the updated component.</p>
                            /// - On failure, responds with [`SdkError<UpdateComponentError>`](crate::error::UpdateComponentError)
    pub fn update_component(&self) -> crate::client::fluent_builders::UpdateComponent {
                                crate::client::fluent_builders::UpdateComponent::new(self.handle.clone())
                            }
}

