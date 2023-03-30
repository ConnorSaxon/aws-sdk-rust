// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeSecret`](crate::client::fluent_builders::DescribeSecret) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`secret_id(impl Into<String>)`](crate::client::fluent_builders::DescribeSecret::secret_id) / [`set_secret_id(Option<String>)`](crate::client::fluent_builders::DescribeSecret::set_secret_id): <p>The ARN or name of the secret. </p>  <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
                            /// - On success, responds with [`DescribeSecretOutput`](crate::output::DescribeSecretOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::output::DescribeSecretOutput::arn): <p>The ARN of the secret.</p>
    ///   - [`name(Option<String>)`](crate::output::DescribeSecretOutput::name): <p>The name of the secret.</p>
    ///   - [`description(Option<String>)`](crate::output::DescribeSecretOutput::description): <p>The description of the secret.</p>
    ///   - [`kms_key_id(Option<String>)`](crate::output::DescribeSecretOutput::kms_key_id): <p>The key ID or alias ARN of the KMS key that Secrets Manager uses to encrypt the secret value. If the secret is encrypted with the Amazon Web Services managed key <code>aws/secretsmanager</code>, this field is omitted. Secrets created using the console use an KMS key ID.</p>
    ///   - [`rotation_enabled(Option<bool>)`](crate::output::DescribeSecretOutput::rotation_enabled): <p>Specifies whether automatic rotation is turned on for this secret.</p>  <p>To turn on rotation, use <code>RotateSecret</code>. To turn off rotation, use <code>CancelRotateSecret</code>.</p>
    ///   - [`rotation_lambda_arn(Option<String>)`](crate::output::DescribeSecretOutput::rotation_lambda_arn): <p>The ARN of the Lambda function that Secrets Manager invokes to rotate the secret. </p>
    ///   - [`rotation_rules(Option<RotationRulesType>)`](crate::output::DescribeSecretOutput::rotation_rules): <p>The rotation schedule and Lambda function for this secret. If the secret previously had rotation turned on, but it is now turned off, this field shows the previous rotation schedule and rotation function. If the secret never had rotation turned on, this field is omitted.</p>
    ///   - [`last_rotated_date(Option<DateTime>)`](crate::output::DescribeSecretOutput::last_rotated_date): <p>The last date and time that Secrets Manager rotated the secret. If the secret isn't configured for rotation, Secrets Manager returns null.</p>
    ///   - [`last_changed_date(Option<DateTime>)`](crate::output::DescribeSecretOutput::last_changed_date): <p>The last date and time that this secret was modified in any way.</p>
    ///   - [`last_accessed_date(Option<DateTime>)`](crate::output::DescribeSecretOutput::last_accessed_date): <p>The date that the secret was last accessed in the Region. This field is omitted if the secret has never been retrieved in the Region.</p>
    ///   - [`deleted_date(Option<DateTime>)`](crate::output::DescribeSecretOutput::deleted_date): <p>The date the secret is scheduled for deletion. If it is not scheduled for deletion, this field is omitted. When you delete a secret, Secrets Manager requires a recovery window of at least 7 days before deleting the secret. Some time after the deleted date, Secrets Manager deletes the secret, including all of its versions.</p>  <p>If a secret is scheduled for deletion, then its details, including the encrypted secret value, is not accessible. To cancel a scheduled deletion and restore access to the secret, use <code>RestoreSecret</code>.</p>
    ///   - [`next_rotation_date(Option<DateTime>)`](crate::output::DescribeSecretOutput::next_rotation_date): <p>The next date and time that Secrets Manager will rotate the secret, rounded to the nearest hour. If the secret isn't configured for rotation, Secrets Manager returns null.</p>
    ///   - [`tags(Option<Vec<Tag>>)`](crate::output::DescribeSecretOutput::tags): <p>The list of tags attached to the secret. To add tags to a secret, use <code>TagResource</code>. To remove tags, use <code>UntagResource</code>.</p>
    ///   - [`version_ids_to_stages(Option<HashMap<String, Vec<String>>>)`](crate::output::DescribeSecretOutput::version_ids_to_stages): <p>A list of the versions of the secret that have staging labels attached. Versions that don't have staging labels are considered deprecated and Secrets Manager can delete them.</p>  <p>Secrets Manager uses staging labels to indicate the status of a secret version during rotation. The three staging labels for rotation are: </p>  <ul>   <li> <p> <code>AWSCURRENT</code>, which indicates the current version of the secret.</p> </li>   <li> <p> <code>AWSPENDING</code>, which indicates the version of the secret that contains new secret information that will become the next current version when rotation finishes.</p> <p>During rotation, Secrets Manager creates an <code>AWSPENDING</code> version ID before creating the new secret version. To check if a secret version exists, call <code>GetSecretValue</code>.</p> </li>   <li> <p> <code>AWSPREVIOUS</code>, which indicates the previous current version of the secret. You can use this as the <i>last known good</i> version.</p> </li>  </ul>  <p>For more information about rotation and staging labels, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html">How rotation works</a>.</p>
    ///   - [`owning_service(Option<String>)`](crate::output::DescribeSecretOutput::owning_service): <p>The ID of the service that created this secret. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/service-linked-secrets.html">Secrets managed by other Amazon Web Services services</a>.</p>
    ///   - [`created_date(Option<DateTime>)`](crate::output::DescribeSecretOutput::created_date): <p>The date the secret was created.</p>
    ///   - [`primary_region(Option<String>)`](crate::output::DescribeSecretOutput::primary_region): <p>The Region the secret is in. If a secret is replicated to other Regions, the replicas are listed in <code>ReplicationStatus</code>. </p>
    ///   - [`replication_status(Option<Vec<ReplicationStatusType>>)`](crate::output::DescribeSecretOutput::replication_status): <p>A list of the replicas of this secret and their status: </p>  <ul>   <li> <p> <code>Failed</code>, which indicates that the replica was not created.</p> </li>   <li> <p> <code>InProgress</code>, which indicates that Secrets Manager is in the process of creating the replica.</p> </li>   <li> <p> <code>InSync</code>, which indicates that the replica was created.</p> </li>  </ul>
                            /// - On failure, responds with [`SdkError<DescribeSecretError>`](crate::error::DescribeSecretError)
    pub fn describe_secret(&self) -> crate::client::fluent_builders::DescribeSecret {
                                crate::client::fluent_builders::DescribeSecret::new(self.handle.clone())
                            }
}

