// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeNotebookInstance`](crate::client::fluent_builders::DescribeNotebookInstance) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`notebook_instance_name(impl Into<String>)`](crate::client::fluent_builders::DescribeNotebookInstance::notebook_instance_name) / [`set_notebook_instance_name(Option<String>)`](crate::client::fluent_builders::DescribeNotebookInstance::set_notebook_instance_name): <p>The name of the notebook instance that you want information about.</p>
                            /// - On success, responds with [`DescribeNotebookInstanceOutput`](crate::output::DescribeNotebookInstanceOutput) with field(s):
    ///   - [`notebook_instance_arn(Option<String>)`](crate::output::DescribeNotebookInstanceOutput::notebook_instance_arn): <p>The Amazon Resource Name (ARN) of the notebook instance.</p>
    ///   - [`notebook_instance_name(Option<String>)`](crate::output::DescribeNotebookInstanceOutput::notebook_instance_name): <p>The name of the SageMaker notebook instance. </p>
    ///   - [`notebook_instance_status(Option<NotebookInstanceStatus>)`](crate::output::DescribeNotebookInstanceOutput::notebook_instance_status): <p>The status of the notebook instance.</p>
    ///   - [`failure_reason(Option<String>)`](crate::output::DescribeNotebookInstanceOutput::failure_reason): <p>If status is <code>Failed</code>, the reason it failed.</p>
    ///   - [`url(Option<String>)`](crate::output::DescribeNotebookInstanceOutput::url): <p>The URL that you use to connect to the Jupyter notebook that is running in your notebook instance. </p>
    ///   - [`instance_type(Option<InstanceType>)`](crate::output::DescribeNotebookInstanceOutput::instance_type): <p>The type of ML compute instance running on the notebook instance.</p>
    ///   - [`subnet_id(Option<String>)`](crate::output::DescribeNotebookInstanceOutput::subnet_id): <p>The ID of the VPC subnet.</p>
    ///   - [`security_groups(Option<Vec<String>>)`](crate::output::DescribeNotebookInstanceOutput::security_groups): <p>The IDs of the VPC security groups.</p>
    ///   - [`role_arn(Option<String>)`](crate::output::DescribeNotebookInstanceOutput::role_arn): <p>The Amazon Resource Name (ARN) of the IAM role associated with the instance. </p>
    ///   - [`kms_key_id(Option<String>)`](crate::output::DescribeNotebookInstanceOutput::kms_key_id): <p>The Amazon Web Services KMS key ID SageMaker uses to encrypt data when storing it on the ML storage volume attached to the instance. </p>
    ///   - [`network_interface_id(Option<String>)`](crate::output::DescribeNotebookInstanceOutput::network_interface_id): <p>The network interface IDs that SageMaker created at the time of creating the instance. </p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::output::DescribeNotebookInstanceOutput::last_modified_time): <p>A timestamp. Use this parameter to retrieve the time when the notebook instance was last modified. </p>
    ///   - [`creation_time(Option<DateTime>)`](crate::output::DescribeNotebookInstanceOutput::creation_time): <p>A timestamp. Use this parameter to return the time when the notebook instance was created</p>
    ///   - [`notebook_instance_lifecycle_config_name(Option<String>)`](crate::output::DescribeNotebookInstanceOutput::notebook_instance_lifecycle_config_name): <p>Returns the name of a notebook instance lifecycle configuration.</p>  <p>For information about notebook instance lifestyle configurations, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/notebook-lifecycle-config.html">Step 2.1: (Optional) Customize a Notebook Instance</a> </p>
    ///   - [`direct_internet_access(Option<DirectInternetAccess>)`](crate::output::DescribeNotebookInstanceOutput::direct_internet_access): <p>Describes whether SageMaker provides internet access to the notebook instance. If this value is set to <i>Disabled</i>, the notebook instance does not have internet access, and cannot connect to SageMaker training and endpoint services.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/appendix-additional-considerations.html#appendix-notebook-and-internet-access">Notebook Instances Are Internet-Enabled by Default</a>.</p>
    ///   - [`volume_size_in_gb(Option<i32>)`](crate::output::DescribeNotebookInstanceOutput::volume_size_in_gb): <p>The size, in GB, of the ML storage volume attached to the notebook instance.</p>
    ///   - [`accelerator_types(Option<Vec<NotebookInstanceAcceleratorType>>)`](crate::output::DescribeNotebookInstanceOutput::accelerator_types): <p>A list of the Elastic Inference (EI) instance types associated with this notebook instance. Currently only one EI instance type can be associated with a notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/ei.html">Using Elastic Inference in Amazon SageMaker</a>.</p>
    ///   - [`default_code_repository(Option<String>)`](crate::output::DescribeNotebookInstanceOutput::default_code_repository): <p>The Git repository associated with the notebook instance as its default code repository. This can be either the name of a Git repository stored as a resource in your account, or the URL of a Git repository in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">Amazon Web Services CodeCommit</a> or in any other Git repository. When you open a notebook instance, it opens in the directory that contains this repository. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with SageMaker Notebook Instances</a>.</p>
    ///   - [`additional_code_repositories(Option<Vec<String>>)`](crate::output::DescribeNotebookInstanceOutput::additional_code_repositories): <p>An array of up to three Git repositories associated with the notebook instance. These can be either the names of Git repositories stored as resources in your account, or the URL of Git repositories in <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html">Amazon Web Services CodeCommit</a> or in any other Git repository. These repositories are cloned at the same level as the default repository of your notebook instance. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/nbi-git-repo.html">Associating Git Repositories with SageMaker Notebook Instances</a>.</p>
    ///   - [`root_access(Option<RootAccess>)`](crate::output::DescribeNotebookInstanceOutput::root_access): <p>Whether root access is enabled or disabled for users of the notebook instance.</p> <note>   <p>Lifecycle configurations need root access to be able to set up a notebook instance. Because of this, lifecycle configurations associated with a notebook instance always run with root access even if you disable root access for users.</p>  </note>
    ///   - [`platform_identifier(Option<String>)`](crate::output::DescribeNotebookInstanceOutput::platform_identifier): <p>The platform identifier of the notebook instance runtime environment.</p>
    ///   - [`instance_metadata_service_configuration(Option<InstanceMetadataServiceConfiguration>)`](crate::output::DescribeNotebookInstanceOutput::instance_metadata_service_configuration): <p>Information on the IMDS configuration of the notebook instance</p>
                            /// - On failure, responds with [`SdkError<DescribeNotebookInstanceError>`](crate::error::DescribeNotebookInstanceError)
    pub fn describe_notebook_instance(&self) -> crate::client::fluent_builders::DescribeNotebookInstance {
                                crate::client::fluent_builders::DescribeNotebookInstance::new(self.handle.clone())
                            }
}

