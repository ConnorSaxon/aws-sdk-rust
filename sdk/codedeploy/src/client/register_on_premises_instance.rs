// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RegisterOnPremisesInstance`](crate::client::fluent_builders::RegisterOnPremisesInstance) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`instance_name(impl Into<String>)`](crate::client::fluent_builders::RegisterOnPremisesInstance::instance_name) / [`set_instance_name(Option<String>)`](crate::client::fluent_builders::RegisterOnPremisesInstance::set_instance_name): <p>The name of the on-premises instance to register.</p>
    ///   - [`iam_session_arn(impl Into<String>)`](crate::client::fluent_builders::RegisterOnPremisesInstance::iam_session_arn) / [`set_iam_session_arn(Option<String>)`](crate::client::fluent_builders::RegisterOnPremisesInstance::set_iam_session_arn): <p>The ARN of the IAM session to associate with the on-premises instance.</p>
    ///   - [`iam_user_arn(impl Into<String>)`](crate::client::fluent_builders::RegisterOnPremisesInstance::iam_user_arn) / [`set_iam_user_arn(Option<String>)`](crate::client::fluent_builders::RegisterOnPremisesInstance::set_iam_user_arn): <p>The ARN of the IAM user to associate with the on-premises instance.</p>
                            /// - On success, responds with [`RegisterOnPremisesInstanceOutput`](crate::output::RegisterOnPremisesInstanceOutput)
                            /// - On failure, responds with [`SdkError<RegisterOnPremisesInstanceError>`](crate::error::RegisterOnPremisesInstanceError)
    pub fn register_on_premises_instance(&self) -> crate::client::fluent_builders::RegisterOnPremisesInstance {
                                crate::client::fluent_builders::RegisterOnPremisesInstance::new(self.handle.clone())
                            }
}

