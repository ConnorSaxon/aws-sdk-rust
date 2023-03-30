// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutLifecycleConfiguration`](crate::client::fluent_builders::PutLifecycleConfiguration) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`file_system_id(impl Into<String>)`](crate::client::fluent_builders::PutLifecycleConfiguration::file_system_id) / [`set_file_system_id(Option<String>)`](crate::client::fluent_builders::PutLifecycleConfiguration::set_file_system_id): <p>The ID of the file system for which you are creating the <code>LifecycleConfiguration</code> object (String).</p>
    ///   - [`lifecycle_policies(Vec<LifecyclePolicy>)`](crate::client::fluent_builders::PutLifecycleConfiguration::lifecycle_policies) / [`set_lifecycle_policies(Option<Vec<LifecyclePolicy>>)`](crate::client::fluent_builders::PutLifecycleConfiguration::set_lifecycle_policies): <p>An array of <code>LifecyclePolicy</code> objects that define the file system's <code>LifecycleConfiguration</code> object. A <code>LifecycleConfiguration</code> object informs EFS lifecycle management and EFS Intelligent-Tiering of the following:</p>  <ul>   <li> <p>When to move files in the file system from primary storage to the IA storage class.</p> </li>   <li> <p>When to move files that are in IA storage to primary storage.</p> </li>  </ul> <note>   <p>When using the <code>put-lifecycle-configuration</code> CLI command or the <code>PutLifecycleConfiguration</code> API action, Amazon EFS requires that each <code>LifecyclePolicy</code> object have only a single transition. This means that in a request body, <code>LifecyclePolicies</code> must be structured as an array of <code>LifecyclePolicy</code> objects, one object for each transition, <code>TransitionToIA</code>, <code>TransitionToPrimaryStorageClass</code>. See the example requests in the following section for more information.</p>  </note>
                            /// - On success, responds with [`PutLifecycleConfigurationOutput`](crate::output::PutLifecycleConfigurationOutput) with field(s):
    ///   - [`lifecycle_policies(Option<Vec<LifecyclePolicy>>)`](crate::output::PutLifecycleConfigurationOutput::lifecycle_policies): <p>An array of lifecycle management policies. EFS supports a maximum of one policy per file system.</p>
                            /// - On failure, responds with [`SdkError<PutLifecycleConfigurationError>`](crate::error::PutLifecycleConfigurationError)
    pub fn put_lifecycle_configuration(&self) -> crate::client::fluent_builders::PutLifecycleConfiguration {
                                crate::client::fluent_builders::PutLifecycleConfiguration::new(self.handle.clone())
                            }
}

