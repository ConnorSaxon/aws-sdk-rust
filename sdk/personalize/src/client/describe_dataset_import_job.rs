// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeDatasetImportJob`](crate::client::fluent_builders::DescribeDatasetImportJob) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`dataset_import_job_arn(impl Into<String>)`](crate::client::fluent_builders::DescribeDatasetImportJob::dataset_import_job_arn) / [`set_dataset_import_job_arn(Option<String>)`](crate::client::fluent_builders::DescribeDatasetImportJob::set_dataset_import_job_arn): <p>The Amazon Resource Name (ARN) of the dataset import job to describe.</p>
                            /// - On success, responds with [`DescribeDatasetImportJobOutput`](crate::output::DescribeDatasetImportJobOutput) with field(s):
    ///   - [`dataset_import_job(Option<DatasetImportJob>)`](crate::output::DescribeDatasetImportJobOutput::dataset_import_job): <p>Information about the dataset import job, including the status.</p>  <p>The status is one of the following values:</p>  <ul>   <li> <p>CREATE PENDING</p> </li>   <li> <p>CREATE IN_PROGRESS</p> </li>   <li> <p>ACTIVE</p> </li>   <li> <p>CREATE FAILED</p> </li>  </ul>
                            /// - On failure, responds with [`SdkError<DescribeDatasetImportJobError>`](crate::error::DescribeDatasetImportJobError)
    pub fn describe_dataset_import_job(&self) -> crate::client::fluent_builders::DescribeDatasetImportJob {
                                crate::client::fluent_builders::DescribeDatasetImportJob::new(self.handle.clone())
                            }
}

