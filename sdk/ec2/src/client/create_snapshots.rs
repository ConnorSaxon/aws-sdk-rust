// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateSnapshots`](crate::client::fluent_builders::CreateSnapshots) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateSnapshots::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateSnapshots::set_description): <p> A description propagated to every snapshot specified by the instance.</p>
    ///   - [`instance_specification(InstanceSpecification)`](crate::client::fluent_builders::CreateSnapshots::instance_specification) / [`set_instance_specification(Option<InstanceSpecification>)`](crate::client::fluent_builders::CreateSnapshots::set_instance_specification): <p>The instance to specify which volumes should be included in the snapshots.</p>
    ///   - [`outpost_arn(impl Into<String>)`](crate::client::fluent_builders::CreateSnapshots::outpost_arn) / [`set_outpost_arn(Option<String>)`](crate::client::fluent_builders::CreateSnapshots::set_outpost_arn): <p>The Amazon Resource Name (ARN) of the Outpost on which to create the local snapshots.</p>  <ul>   <li> <p>To create snapshots from an instance in a Region, omit this parameter. The snapshots are created in the same Region as the instance.</p> </li>   <li> <p>To create snapshots from an instance on an Outpost and store the snapshots in the Region, omit this parameter. The snapshots are created in the Region for the Outpost.</p> </li>   <li> <p>To create snapshots from an instance on an Outpost and store the snapshots on an Outpost, specify the ARN of the destination Outpost. The snapshots must be created on the same Outpost as the instance.</p> </li>  </ul>  <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#create-multivol-snapshot"> Create multi-volume local snapshots from instances on an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::client::fluent_builders::CreateSnapshots::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::client::fluent_builders::CreateSnapshots::set_tag_specifications): <p>Tags to apply to every snapshot specified by the instance.</p>
    ///   - [`dry_run(bool)`](crate::client::fluent_builders::CreateSnapshots::dry_run) / [`set_dry_run(Option<bool>)`](crate::client::fluent_builders::CreateSnapshots::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`copy_tags_from_source(CopyTagsFromSource)`](crate::client::fluent_builders::CreateSnapshots::copy_tags_from_source) / [`set_copy_tags_from_source(Option<CopyTagsFromSource>)`](crate::client::fluent_builders::CreateSnapshots::set_copy_tags_from_source): <p>Copies the tags from the specified volume to corresponding snapshot.</p>
                            /// - On success, responds with [`CreateSnapshotsOutput`](crate::output::CreateSnapshotsOutput) with field(s):
    ///   - [`snapshots(Option<Vec<SnapshotInfo>>)`](crate::output::CreateSnapshotsOutput::snapshots): <p>List of snapshots.</p>
                            /// - On failure, responds with [`SdkError<CreateSnapshotsError>`](crate::error::CreateSnapshotsError)
    pub fn create_snapshots(&self) -> crate::client::fluent_builders::CreateSnapshots {
                                crate::client::fluent_builders::CreateSnapshots::new(self.handle.clone())
                            }
}

