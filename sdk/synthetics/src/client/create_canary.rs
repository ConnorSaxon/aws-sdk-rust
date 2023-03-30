// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateCanary`](crate::client::fluent_builders::CreateCanary) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateCanary::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateCanary::set_name): <p>The name for this canary. Be sure to give it a descriptive name that distinguishes it from other canaries in your account.</p>  <p>Do not include secrets or proprietary information in your canary names. The canary name makes up part of the canary ARN, and the ARN is included in outbound calls over the internet. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/servicelens_canaries_security.html">Security Considerations for Synthetics Canaries</a>.</p>
    ///   - [`code(CanaryCodeInput)`](crate::client::fluent_builders::CreateCanary::code) / [`set_code(Option<CanaryCodeInput>)`](crate::client::fluent_builders::CreateCanary::set_code): <p>A structure that includes the entry point from which the canary should start running your script. If the script is stored in an S3 bucket, the bucket name, key, and version are also included. </p>
    ///   - [`artifact_s3_location(impl Into<String>)`](crate::client::fluent_builders::CreateCanary::artifact_s3_location) / [`set_artifact_s3_location(Option<String>)`](crate::client::fluent_builders::CreateCanary::set_artifact_s3_location): <p>The location in Amazon S3 where Synthetics stores artifacts from the test runs of this canary. Artifacts include the log file, screenshots, and HAR files. The name of the S3 bucket can't include a period (.).</p>
    ///   - [`execution_role_arn(impl Into<String>)`](crate::client::fluent_builders::CreateCanary::execution_role_arn) / [`set_execution_role_arn(Option<String>)`](crate::client::fluent_builders::CreateCanary::set_execution_role_arn): <p>The ARN of the IAM role to be used to run the canary. This role must already exist, and must include <code>lambda.amazonaws.com</code> as a principal in the trust policy. The role must also have the following permissions:</p>  <ul>   <li> <p> <code>s3:PutObject</code> </p> </li>   <li> <p> <code>s3:GetBucketLocation</code> </p> </li>   <li> <p> <code>s3:ListAllMyBuckets</code> </p> </li>   <li> <p> <code>cloudwatch:PutMetricData</code> </p> </li>   <li> <p> <code>logs:CreateLogGroup</code> </p> </li>   <li> <p> <code>logs:CreateLogStream</code> </p> </li>   <li> <p> <code>logs:PutLogEvents</code> </p> </li>  </ul>
    ///   - [`schedule(CanaryScheduleInput)`](crate::client::fluent_builders::CreateCanary::schedule) / [`set_schedule(Option<CanaryScheduleInput>)`](crate::client::fluent_builders::CreateCanary::set_schedule): <p>A structure that contains information about how often the canary is to run and when these test runs are to stop.</p>
    ///   - [`run_config(CanaryRunConfigInput)`](crate::client::fluent_builders::CreateCanary::run_config) / [`set_run_config(Option<CanaryRunConfigInput>)`](crate::client::fluent_builders::CreateCanary::set_run_config): <p>A structure that contains the configuration for individual canary runs, such as timeout value and environment variables.</p> <important>   <p>The environment variables keys and values are not encrypted. Do not store sensitive information in this field.</p>  </important>
    ///   - [`success_retention_period_in_days(i32)`](crate::client::fluent_builders::CreateCanary::success_retention_period_in_days) / [`set_success_retention_period_in_days(Option<i32>)`](crate::client::fluent_builders::CreateCanary::set_success_retention_period_in_days): <p>The number of days to retain data about successful runs of this canary. If you omit this field, the default of 31 days is used. The valid range is 1 to 455 days.</p>
    ///   - [`failure_retention_period_in_days(i32)`](crate::client::fluent_builders::CreateCanary::failure_retention_period_in_days) / [`set_failure_retention_period_in_days(Option<i32>)`](crate::client::fluent_builders::CreateCanary::set_failure_retention_period_in_days): <p>The number of days to retain data about failed runs of this canary. If you omit this field, the default of 31 days is used. The valid range is 1 to 455 days.</p>
    ///   - [`runtime_version(impl Into<String>)`](crate::client::fluent_builders::CreateCanary::runtime_version) / [`set_runtime_version(Option<String>)`](crate::client::fluent_builders::CreateCanary::set_runtime_version): <p>Specifies the runtime version to use for the canary. For a list of valid runtime versions and more information about runtime versions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html"> Canary Runtime Versions</a>.</p>
    ///   - [`vpc_config(VpcConfigInput)`](crate::client::fluent_builders::CreateCanary::vpc_config) / [`set_vpc_config(Option<VpcConfigInput>)`](crate::client::fluent_builders::CreateCanary::set_vpc_config): <p>If this canary is to test an endpoint in a VPC, this structure contains information about the subnet and security groups of the VPC endpoint. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_VPC.html"> Running a Canary in a VPC</a>.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateCanary::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateCanary::set_tags): <p>A list of key-value pairs to associate with the canary. You can associate as many as 50 tags with a canary.</p>  <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to access or change only the resources that have certain tag values.</p>
    ///   - [`artifact_config(ArtifactConfigInput)`](crate::client::fluent_builders::CreateCanary::artifact_config) / [`set_artifact_config(Option<ArtifactConfigInput>)`](crate::client::fluent_builders::CreateCanary::set_artifact_config): <p>A structure that contains the configuration for canary artifacts, including the encryption-at-rest settings for artifacts that the canary uploads to Amazon S3.</p>
                            /// - On success, responds with [`CreateCanaryOutput`](crate::output::CreateCanaryOutput) with field(s):
    ///   - [`canary(Option<Canary>)`](crate::output::CreateCanaryOutput::canary): <p>The full details about the canary you have created.</p>
                            /// - On failure, responds with [`SdkError<CreateCanaryError>`](crate::error::CreateCanaryError)
    pub fn create_canary(&self) -> crate::client::fluent_builders::CreateCanary {
                                crate::client::fluent_builders::CreateCanary::new(self.handle.clone())
                            }
}

