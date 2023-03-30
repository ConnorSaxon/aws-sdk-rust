// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ExportEBSVolumeRecommendations`](crate::client::fluent_builders::ExportEBSVolumeRecommendations) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`account_ids(Vec<String>)`](crate::client::fluent_builders::ExportEBSVolumeRecommendations::account_ids) / [`set_account_ids(Option<Vec<String>>)`](crate::client::fluent_builders::ExportEBSVolumeRecommendations::set_account_ids): <p>The IDs of the Amazon Web Services accounts for which to export Amazon EBS volume recommendations.</p>  <p>If your account is the management account of an organization, use this parameter to specify the member account for which you want to export recommendations.</p>  <p>This parameter cannot be specified together with the include member accounts parameter. The parameters are mutually exclusive.</p>  <p>Recommendations for member accounts are not included in the export if this parameter, or the include member accounts parameter, is omitted.</p>  <p>You can specify multiple account IDs per request.</p>
    ///   - [`filters(Vec<EbsFilter>)`](crate::client::fluent_builders::ExportEBSVolumeRecommendations::filters) / [`set_filters(Option<Vec<EbsFilter>>)`](crate::client::fluent_builders::ExportEBSVolumeRecommendations::set_filters): <p>An array of objects to specify a filter that exports a more specific set of Amazon EBS volume recommendations.</p>
    ///   - [`fields_to_export(Vec<ExportableVolumeField>)`](crate::client::fluent_builders::ExportEBSVolumeRecommendations::fields_to_export) / [`set_fields_to_export(Option<Vec<ExportableVolumeField>>)`](crate::client::fluent_builders::ExportEBSVolumeRecommendations::set_fields_to_export): <p>The recommendations data to include in the export file. For more information about the fields that can be exported, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/exporting-recommendations.html#exported-files">Exported files</a> in the <i>Compute Optimizer User Guide</i>.</p>
    ///   - [`s3_destination_config(S3DestinationConfig)`](crate::client::fluent_builders::ExportEBSVolumeRecommendations::s3_destination_config) / [`set_s3_destination_config(Option<S3DestinationConfig>)`](crate::client::fluent_builders::ExportEBSVolumeRecommendations::set_s3_destination_config): <p>Describes the destination Amazon Simple Storage Service (Amazon S3) bucket name and key prefix for a recommendations export job.</p>  <p>You must create the destination Amazon S3 bucket for your recommendations export before you create the export job. Compute Optimizer does not create the S3 bucket for you. After you create the S3 bucket, ensure that it has the required permission policy to allow Compute Optimizer to write the export file to it. If you plan to specify an object prefix when you create the export job, you must include the object prefix in the policy that you add to the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/create-s3-bucket-policy-for-compute-optimizer.html">Amazon S3 Bucket Policy for Compute Optimizer</a> in the <i>Compute Optimizer User Guide</i>.</p>
    ///   - [`file_format(FileFormat)`](crate::client::fluent_builders::ExportEBSVolumeRecommendations::file_format) / [`set_file_format(Option<FileFormat>)`](crate::client::fluent_builders::ExportEBSVolumeRecommendations::set_file_format): <p>The format of the export file.</p>  <p>The only export file format currently supported is <code>Csv</code>.</p>
    ///   - [`include_member_accounts(bool)`](crate::client::fluent_builders::ExportEBSVolumeRecommendations::include_member_accounts) / [`set_include_member_accounts(bool)`](crate::client::fluent_builders::ExportEBSVolumeRecommendations::set_include_member_accounts): <p>Indicates whether to include recommendations for resources in all member accounts of the organization if your account is the management account of an organization.</p>  <p>The member accounts must also be opted in to Compute Optimizer, and trusted access for Compute Optimizer must be enabled in the organization account. For more information, see <a href="https://docs.aws.amazon.com/compute-optimizer/latest/ug/security-iam.html#trusted-service-access">Compute Optimizer and Amazon Web Services Organizations trusted access</a> in the <i>Compute Optimizer User Guide</i>.</p>  <p>Recommendations for member accounts of the organization are not included in the export file if this parameter is omitted.</p>  <p>This parameter cannot be specified together with the account IDs parameter. The parameters are mutually exclusive.</p>  <p>Recommendations for member accounts are not included in the export if this parameter, or the account IDs parameter, is omitted.</p>
                            /// - On success, responds with [`ExportEbsVolumeRecommendationsOutput`](crate::output::ExportEbsVolumeRecommendationsOutput) with field(s):
    ///   - [`job_id(Option<String>)`](crate::output::ExportEbsVolumeRecommendationsOutput::job_id): <p>The identification number of the export job.</p>  <p>Use the <code>DescribeRecommendationExportJobs</code> action, and specify the job ID to view the status of an export job.</p>
    ///   - [`s3_destination(Option<S3Destination>)`](crate::output::ExportEbsVolumeRecommendationsOutput::s3_destination): <p>Describes the destination Amazon Simple Storage Service (Amazon S3) bucket name and object keys of a recommendations export file, and its associated metadata file.</p>
                            /// - On failure, responds with [`SdkError<ExportEBSVolumeRecommendationsError>`](crate::error::ExportEBSVolumeRecommendationsError)
    pub fn export_ebs_volume_recommendations(&self) -> crate::client::fluent_builders::ExportEBSVolumeRecommendations {
                                crate::client::fluent_builders::ExportEBSVolumeRecommendations::new(self.handle.clone())
                            }
}

