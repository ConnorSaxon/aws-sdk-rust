// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetDataSource`](crate::client::fluent_builders::GetDataSource) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`data_source_id(impl Into<String>)`](crate::client::fluent_builders::GetDataSource::data_source_id) / [`set_data_source_id(Option<String>)`](crate::client::fluent_builders::GetDataSource::set_data_source_id): <p>The ID assigned to the <code>DataSource</code> at creation.</p>
    ///   - [`verbose(bool)`](crate::client::fluent_builders::GetDataSource::verbose) / [`set_verbose(bool)`](crate::client::fluent_builders::GetDataSource::set_verbose): <p>Specifies whether the <code>GetDataSource</code> operation should return <code>DataSourceSchema</code>.</p>  <p>If true, <code>DataSourceSchema</code> is returned.</p>  <p>If false, <code>DataSourceSchema</code> is not returned.</p>
                            /// - On success, responds with [`GetDataSourceOutput`](crate::output::GetDataSourceOutput) with field(s):
    ///   - [`data_source_id(Option<String>)`](crate::output::GetDataSourceOutput::data_source_id): <p>The ID assigned to the <code>DataSource</code> at creation. This value should be identical to the value of the <code>DataSourceId</code> in the request.</p>
    ///   - [`data_location_s3(Option<String>)`](crate::output::GetDataSourceOutput::data_location_s3): <p>The location of the data file or directory in Amazon Simple Storage Service (Amazon S3).</p>
    ///   - [`data_rearrangement(Option<String>)`](crate::output::GetDataSourceOutput::data_rearrangement): <p>A JSON string that represents the splitting and rearrangement requirement used when this <code>DataSource</code> was created.</p>
    ///   - [`created_by_iam_user(Option<String>)`](crate::output::GetDataSourceOutput::created_by_iam_user): <p>The AWS user account from which the <code>DataSource</code> was created. The account type can be either an AWS root account or an AWS Identity and Access Management (IAM) user account.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::output::GetDataSourceOutput::created_at): <p>The time that the <code>DataSource</code> was created. The time is expressed in epoch time.</p>
    ///   - [`last_updated_at(Option<DateTime>)`](crate::output::GetDataSourceOutput::last_updated_at): <p>The time of the most recent edit to the <code>DataSource</code>. The time is expressed in epoch time.</p>
    ///   - [`data_size_in_bytes(Option<i64>)`](crate::output::GetDataSourceOutput::data_size_in_bytes): <p>The total size of observations in the data files.</p>
    ///   - [`number_of_files(Option<i64>)`](crate::output::GetDataSourceOutput::number_of_files): <p>The number of data files referenced by the <code>DataSource</code>.</p>
    ///   - [`name(Option<String>)`](crate::output::GetDataSourceOutput::name): <p>A user-supplied name or description of the <code>DataSource</code>.</p>
    ///   - [`status(Option<EntityStatus>)`](crate::output::GetDataSourceOutput::status): <p>The current status of the <code>DataSource</code>. This element can have one of the following values:</p>  <ul>   <li> <p> <code>PENDING</code> - Amazon ML submitted a request to create a <code>DataSource</code>.</p> </li>   <li> <p> <code>INPROGRESS</code> - The creation process is underway.</p> </li>   <li> <p> <code>FAILED</code> - The request to create a <code>DataSource</code> did not run to completion. It is not usable.</p> </li>   <li> <p> <code>COMPLETED</code> - The creation process completed successfully.</p> </li>   <li> <p> <code>DELETED</code> - The <code>DataSource</code> is marked as deleted. It is not usable.</p> </li>  </ul>
    ///   - [`log_uri(Option<String>)`](crate::output::GetDataSourceOutput::log_uri): <p>A link to the file containing logs of <code>CreateDataSourceFrom*</code> operations.</p>
    ///   - [`message(Option<String>)`](crate::output::GetDataSourceOutput::message): <p>The user-supplied description of the most recent details about creating the <code>DataSource</code>.</p>
    ///   - [`redshift_metadata(Option<RedshiftMetadata>)`](crate::output::GetDataSourceOutput::redshift_metadata): <p>Describes the <code>DataSource</code> details specific to Amazon Redshift.</p>
    ///   - [`rds_metadata(Option<RdsMetadata>)`](crate::output::GetDataSourceOutput::rds_metadata): <p>The datasource details that are specific to Amazon RDS.</p>
    ///   - [`role_arn(Option<String>)`](crate::output::GetDataSourceOutput::role_arn): <p>The Amazon Resource Name (ARN) of an <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/roles-toplevel.html#roles-about-termsandconcepts">AWS IAM Role</a>, such as the following: arn:aws:iam::account:role/rolename. </p>
    ///   - [`compute_statistics(bool)`](crate::output::GetDataSourceOutput::compute_statistics): <p> The parameter is <code>true</code> if statistics need to be generated from the observation data. </p>
    ///   - [`compute_time(Option<i64>)`](crate::output::GetDataSourceOutput::compute_time): <p>The approximate CPU time in milliseconds that Amazon Machine Learning spent processing the <code>DataSource</code>, normalized and scaled on computation resources. <code>ComputeTime</code> is only available if the <code>DataSource</code> is in the <code>COMPLETED</code> state and the <code>ComputeStatistics</code> is set to true.</p>
    ///   - [`finished_at(Option<DateTime>)`](crate::output::GetDataSourceOutput::finished_at): <p>The epoch time when Amazon Machine Learning marked the <code>DataSource</code> as <code>COMPLETED</code> or <code>FAILED</code>. <code>FinishedAt</code> is only available when the <code>DataSource</code> is in the <code>COMPLETED</code> or <code>FAILED</code> state.</p>
    ///   - [`started_at(Option<DateTime>)`](crate::output::GetDataSourceOutput::started_at): <p>The epoch time when Amazon Machine Learning marked the <code>DataSource</code> as <code>INPROGRESS</code>. <code>StartedAt</code> isn't available if the <code>DataSource</code> is in the <code>PENDING</code> state.</p>
    ///   - [`data_source_schema(Option<String>)`](crate::output::GetDataSourceOutput::data_source_schema): <p>The schema used by all of the data files of this <code>DataSource</code>.</p>  <p> <b>Note:</b> This parameter is provided as part of the verbose format.</p>
                            /// - On failure, responds with [`SdkError<GetDataSourceError>`](crate::error::GetDataSourceError)
    pub fn get_data_source(&self) -> crate::client::fluent_builders::GetDataSource {
                                crate::client::fluent_builders::GetDataSource::new(self.handle.clone())
                            }
}

