// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RequestEnvironmentInfo`](crate::client::fluent_builders::RequestEnvironmentInfo) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`environment_id(impl Into<String>)`](crate::client::fluent_builders::RequestEnvironmentInfo::environment_id) / [`set_environment_id(Option<String>)`](crate::client::fluent_builders::RequestEnvironmentInfo::set_environment_id): <p>The ID of the environment of the requested data.</p>  <p>If no such environment is found, <code>RequestEnvironmentInfo</code> returns an <code>InvalidParameterValue</code> error. </p>  <p>Condition: You must specify either this or an EnvironmentName, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    ///   - [`environment_name(impl Into<String>)`](crate::client::fluent_builders::RequestEnvironmentInfo::environment_name) / [`set_environment_name(Option<String>)`](crate::client::fluent_builders::RequestEnvironmentInfo::set_environment_name): <p>The name of the environment of the requested data.</p>  <p>If no such environment is found, <code>RequestEnvironmentInfo</code> returns an <code>InvalidParameterValue</code> error. </p>  <p>Condition: You must specify either this or an EnvironmentId, or both. If you do not specify either, AWS Elastic Beanstalk returns <code>MissingRequiredParameter</code> error. </p>
    ///   - [`info_type(EnvironmentInfoType)`](crate::client::fluent_builders::RequestEnvironmentInfo::info_type) / [`set_info_type(Option<EnvironmentInfoType>)`](crate::client::fluent_builders::RequestEnvironmentInfo::set_info_type): <p>The type of information to request.</p>
                            /// - On success, responds with [`RequestEnvironmentInfoOutput`](crate::output::RequestEnvironmentInfoOutput)
                            /// - On failure, responds with [`SdkError<RequestEnvironmentInfoError>`](crate::error::RequestEnvironmentInfoError)
    pub fn request_environment_info(&self) -> crate::client::fluent_builders::RequestEnvironmentInfo {
                                crate::client::fluent_builders::RequestEnvironmentInfo::new(self.handle.clone())
                            }
}

