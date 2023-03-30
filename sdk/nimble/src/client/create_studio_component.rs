// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateStudioComponent`](crate::client::fluent_builders::CreateStudioComponent) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateStudioComponent::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateStudioComponent::set_client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    ///   - [`configuration(StudioComponentConfiguration)`](crate::client::fluent_builders::CreateStudioComponent::configuration) / [`set_configuration(Option<StudioComponentConfiguration>)`](crate::client::fluent_builders::CreateStudioComponent::set_configuration): <p>The configuration of the studio component, based on component type.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateStudioComponent::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateStudioComponent::set_description): <p>The description.</p>
    ///   - [`ec2_security_group_ids(Vec<String>)`](crate::client::fluent_builders::CreateStudioComponent::ec2_security_group_ids) / [`set_ec2_security_group_ids(Option<Vec<String>>)`](crate::client::fluent_builders::CreateStudioComponent::set_ec2_security_group_ids): <p>The EC2 security groups that control access to the studio component.</p>
    ///   - [`initialization_scripts(Vec<StudioComponentInitializationScript>)`](crate::client::fluent_builders::CreateStudioComponent::initialization_scripts) / [`set_initialization_scripts(Option<Vec<StudioComponentInitializationScript>>)`](crate::client::fluent_builders::CreateStudioComponent::set_initialization_scripts): <p>Initialization scripts for studio components.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateStudioComponent::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateStudioComponent::set_name): <p>The name for the studio component.</p>
    ///   - [`script_parameters(Vec<ScriptParameterKeyValue>)`](crate::client::fluent_builders::CreateStudioComponent::script_parameters) / [`set_script_parameters(Option<Vec<ScriptParameterKeyValue>>)`](crate::client::fluent_builders::CreateStudioComponent::set_script_parameters): <p>Parameters for the studio component scripts.</p>
    ///   - [`studio_id(impl Into<String>)`](crate::client::fluent_builders::CreateStudioComponent::studio_id) / [`set_studio_id(Option<String>)`](crate::client::fluent_builders::CreateStudioComponent::set_studio_id): <p>The studio ID. </p>
    ///   - [`subtype(StudioComponentSubtype)`](crate::client::fluent_builders::CreateStudioComponent::subtype) / [`set_subtype(Option<StudioComponentSubtype>)`](crate::client::fluent_builders::CreateStudioComponent::set_subtype): <p>The specific subtype of a studio component.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateStudioComponent::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateStudioComponent::set_tags): <p>A collection of labels, in the form of key-value pairs, that apply to this resource.</p>
    ///   - [`r#type(StudioComponentType)`](crate::client::fluent_builders::CreateStudioComponent::type) / [`set_type(Option<StudioComponentType>)`](crate::client::fluent_builders::CreateStudioComponent::set_type): <p>The type of the studio component.</p>
    ///   - [`secure_initialization_role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateStudioComponent::secure_initialization_role_arn) / [`set_secure_initialization_role_arn(Option<String>)`](crate::client::fluent_builders::CreateStudioComponent::set_secure_initialization_role_arn): <p>An IAM role attached to Studio Component when the system initialization script runs which give the studio component access to Amazon Web Services resources when the system initialization script runs.</p>
    ///   - [`runtime_role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateStudioComponent::runtime_role_arn) / [`set_runtime_role_arn(Option<String>)`](crate::client::fluent_builders::CreateStudioComponent::set_runtime_role_arn): <p>An IAM role attached to a Studio Component that gives the studio component access to Amazon Web Services resources at anytime while the instance is running. </p>
                            /// - On success, responds with [`CreateStudioComponentOutput`](crate::output::CreateStudioComponentOutput) with field(s):
    ///   - [`studio_component(Option<StudioComponent>)`](crate::output::CreateStudioComponentOutput::studio_component): <p>Information about the studio component.</p>
                            /// - On failure, responds with [`SdkError<CreateStudioComponentError>`](crate::error::CreateStudioComponentError)
    pub fn create_studio_component(&self) -> crate::client::fluent_builders::CreateStudioComponent {
                                crate::client::fluent_builders::CreateStudioComponent::new(self.handle.clone())
                            }
}

