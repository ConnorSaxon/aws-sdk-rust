// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DeleteModelPackageGroupPolicy`](crate::client::fluent_builders::DeleteModelPackageGroupPolicy) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`model_package_group_name(impl Into<String>)`](crate::client::fluent_builders::DeleteModelPackageGroupPolicy::model_package_group_name) / [`set_model_package_group_name(Option<String>)`](crate::client::fluent_builders::DeleteModelPackageGroupPolicy::set_model_package_group_name): <p>The name of the model group for which to delete the policy.</p>
                            /// - On success, responds with [`DeleteModelPackageGroupPolicyOutput`](crate::output::DeleteModelPackageGroupPolicyOutput)
                            /// - On failure, responds with [`SdkError<DeleteModelPackageGroupPolicyError>`](crate::error::DeleteModelPackageGroupPolicyError)
    pub fn delete_model_package_group_policy(&self) -> crate::client::fluent_builders::DeleteModelPackageGroupPolicy {
                                crate::client::fluent_builders::DeleteModelPackageGroupPolicy::new(self.handle.clone())
                            }
}

