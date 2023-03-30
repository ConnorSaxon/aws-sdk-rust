// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeStorediSCSIVolumes`](crate::client::fluent_builders::DescribeStorediSCSIVolumes) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`volume_ar_ns(Vec<String>)`](crate::client::fluent_builders::DescribeStorediSCSIVolumes::volume_ar_ns) / [`set_volume_ar_ns(Option<Vec<String>>)`](crate::client::fluent_builders::DescribeStorediSCSIVolumes::set_volume_ar_ns): <p>An array of strings where each string represents the Amazon Resource Name (ARN) of a stored volume. All of the specified stored volumes must be from the same gateway. Use <code>ListVolumes</code> to get volume ARNs for a gateway.</p>
                            /// - On success, responds with [`DescribeStorediScsiVolumesOutput`](crate::output::DescribeStorediScsiVolumesOutput) with field(s):
    ///   - [`storedi_scsi_volumes(Option<Vec<StorediScsiVolume>>)`](crate::output::DescribeStorediScsiVolumesOutput::storedi_scsi_volumes): <p>Describes a single unit of output from <code>DescribeStorediSCSIVolumes</code>. The following fields are returned:</p>  <ul>   <li> <p> <code>ChapEnabled</code>: Indicates whether mutual CHAP is enabled for the iSCSI target.</p> </li>   <li> <p> <code>LunNumber</code>: The logical disk number.</p> </li>   <li> <p> <code>NetworkInterfaceId</code>: The network interface ID of the stored volume that initiator use to map the stored volume as an iSCSI target.</p> </li>   <li> <p> <code>NetworkInterfacePort</code>: The port used to communicate with iSCSI targets.</p> </li>   <li> <p> <code>PreservedExistingData</code>: Indicates when the stored volume was created, existing data on the underlying local disk was preserved.</p> </li>   <li> <p> <code>SourceSnapshotId</code>: If the stored volume was created from a snapshot, this field contains the snapshot ID used, e.g. <code>snap-1122aabb</code>. Otherwise, this field is not included.</p> </li>   <li> <p> <code>StorediSCSIVolumes</code>: An array of StorediSCSIVolume objects where each object contains metadata about one stored volume.</p> </li>   <li> <p> <code>TargetARN</code>: The Amazon Resource Name (ARN) of the volume target.</p> </li>   <li> <p> <code>VolumeARN</code>: The Amazon Resource Name (ARN) of the stored volume.</p> </li>   <li> <p> <code>VolumeDiskId</code>: The disk ID of the local disk that was specified in the <code>CreateStorediSCSIVolume</code> operation.</p> </li>   <li> <p> <code>VolumeId</code>: The unique identifier of the storage volume, e.g. <code>vol-1122AABB</code>.</p> </li>   <li> <p> <code>VolumeiSCSIAttributes</code>: An <code>VolumeiSCSIAttributes</code> object that represents a collection of iSCSI attributes for one stored volume.</p> </li>   <li> <p> <code>VolumeProgress</code>: Represents the percentage complete if the volume is restoring or bootstrapping that represents the percent of data transferred. This field does not appear in the response if the stored volume is not restoring or bootstrapping.</p> </li>   <li> <p> <code>VolumeSizeInBytes</code>: The size of the volume in bytes.</p> </li>   <li> <p> <code>VolumeStatus</code>: One of the <code>VolumeStatus</code> values that indicates the state of the volume.</p> </li>   <li> <p> <code>VolumeType</code>: One of the enumeration values describing the type of the volume. Currently, only <code>STORED</code> volumes are supported.</p> </li>  </ul>
                            /// - On failure, responds with [`SdkError<DescribeStorediSCSIVolumesError>`](crate::error::DescribeStorediSCSIVolumesError)
    pub fn describe_storedi_scsi_volumes(&self) -> crate::client::fluent_builders::DescribeStorediSCSIVolumes {
                                crate::client::fluent_builders::DescribeStorediSCSIVolumes::new(self.handle.clone())
                            }
}

