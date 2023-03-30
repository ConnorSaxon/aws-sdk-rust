// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetEnvironment`](crate::client::fluent_builders::GetEnvironment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`environment_id(impl Into<String>)`](crate::client::fluent_builders::GetEnvironment::environment_id) / [`set_environment_id(Option<String>)`](crate::client::fluent_builders::GetEnvironment::set_environment_id): <p>The unique identifier of the runtime environment.</p>
                            /// - On success, responds with [`GetEnvironmentOutput`](crate::output::GetEnvironmentOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::GetEnvironmentOutput::name): <p>The name of the runtime environment. Must be unique within the account.</p>
    ///   - [`description(Option<String>)`](crate::output::GetEnvironmentOutput::description): <p>The description of the runtime environment.</p>
    ///   - [`environment_arn(Option<String>)`](crate::output::GetEnvironmentOutput::environment_arn): <p>The Amazon Resource Name (ARN) of the runtime environment.</p>
    ///   - [`environment_id(Option<String>)`](crate::output::GetEnvironmentOutput::environment_id): <p>The unique identifier of the runtime environment.</p>
    ///   - [`instance_type(Option<String>)`](crate::output::GetEnvironmentOutput::instance_type): <p>The type of instance underlying the runtime environment.</p>
    ///   - [`status(Option<EnvironmentLifecycle>)`](crate::output::GetEnvironmentOutput::status): <p>The status of the runtime environment.</p>
    ///   - [`engine_type(Option<EngineType>)`](crate::output::GetEnvironmentOutput::engine_type): <p>The target platform for the runtime environment.</p>
    ///   - [`engine_version(Option<String>)`](crate::output::GetEnvironmentOutput::engine_version): <p>The version of the runtime engine.</p>
    ///   - [`vpc_id(Option<String>)`](crate::output::GetEnvironmentOutput::vpc_id): <p>The unique identifier for the VPC used with this runtime environment.</p>
    ///   - [`subnet_ids(Option<Vec<String>>)`](crate::output::GetEnvironmentOutput::subnet_ids): <p>The unique identifiers of the subnets assigned to this runtime environment.</p>
    ///   - [`security_group_ids(Option<Vec<String>>)`](crate::output::GetEnvironmentOutput::security_group_ids): <p>The unique identifiers of the security groups assigned to this runtime environment.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::GetEnvironmentOutput::creation_time): <p>The timestamp when the runtime environment was created.</p>
    ///   - [`storage_configurations(Option<Vec<StorageConfiguration>>)`](crate::output::GetEnvironmentOutput::storage_configurations): <p>The storage configurations defined for the runtime environment.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::output::GetEnvironmentOutput::tags): <p>The tags defined for this runtime environment.</p>
    ///   - [`high_availability_config(Option<HighAvailabilityConfig>)`](crate::output::GetEnvironmentOutput::high_availability_config): <p>The desired capacity of the high availability configuration for the runtime environment.</p>
    ///   - [`publicly_accessible(bool)`](crate::output::GetEnvironmentOutput::publicly_accessible): <p>Whether applications running in this runtime environment are publicly accessible. </p>
    ///   - [`actual_capacity(Option<i32>)`](crate::output::GetEnvironmentOutput::actual_capacity): <p>The number of instances included in the runtime environment. A standalone runtime environment has a maxiumum of one instance. Currently, a high availability runtime environment has a maximum of two instances. </p>
    ///   - [`load_balancer_arn(Option<String>)`](crate::output::GetEnvironmentOutput::load_balancer_arn): <p>The Amazon Resource Name (ARN) for the load balancer used with the runtime environment.</p>
    ///   - [`status_reason(Option<String>)`](crate::output::GetEnvironmentOutput::status_reason): <p>The reason for the reported status.</p>
    ///   - [`preferred_maintenance_window(Option<String>)`](crate::output::GetEnvironmentOutput::preferred_maintenance_window): <p>Configures the maintenance window you want for the runtime environment. If you do not provide a value, a random system-generated value will be assigned.</p>
    ///   - [`pending_maintenance(Option<PendingMaintenance>)`](crate::output::GetEnvironmentOutput::pending_maintenance): <p>Indicates the pending maintenance scheduled on this environment.</p>
    ///   - [`kms_key_id(Option<String>)`](crate::output::GetEnvironmentOutput::kms_key_id): <p>The identifier of a customer managed key.</p>
                            /// - On failure, responds with [`SdkError<GetEnvironmentError>`](crate::error::GetEnvironmentError)
    pub fn get_environment(&self) -> crate::client::fluent_builders::GetEnvironment {
                                crate::client::fluent_builders::GetEnvironment::new(self.handle.clone())
                            }
}

