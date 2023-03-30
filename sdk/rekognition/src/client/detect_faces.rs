// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DetectFaces`](crate::client::fluent_builders::DetectFaces) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`image(Image)`](crate::client::fluent_builders::DetectFaces::image) / [`set_image(Option<Image>)`](crate::client::fluent_builders::DetectFaces::set_image): <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p>  <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    ///   - [`attributes(Vec<Attribute>)`](crate::client::fluent_builders::DetectFaces::attributes) / [`set_attributes(Option<Vec<Attribute>>)`](crate::client::fluent_builders::DetectFaces::set_attributes): <p>An array of facial attributes you want to be returned. This can be the default list of attributes or all attributes. If you don't specify a value for <code>Attributes</code> or if you specify <code>["DEFAULT"]</code>, the API returns the following subset of facial attributes: <code>BoundingBox</code>, <code>Confidence</code>, <code>Pose</code>, <code>Quality</code>, and <code>Landmarks</code>. If you provide <code>["ALL"]</code>, all facial attributes are returned, but the operation takes longer to complete.</p>  <p>If you provide both, <code>["ALL", "DEFAULT"]</code>, the service uses a logical AND operator to determine which attributes to return (in this case, all attributes). </p>
                            /// - On success, responds with [`DetectFacesOutput`](crate::output::DetectFacesOutput) with field(s):
    ///   - [`face_details(Option<Vec<FaceDetail>>)`](crate::output::DetectFacesOutput::face_details): <p>Details of each face found in the image. </p>
    ///   - [`orientation_correction(Option<OrientationCorrection>)`](crate::output::DetectFacesOutput::orientation_correction): <p>The value of <code>OrientationCorrection</code> is always null.</p>  <p>If the input image is in .jpeg format, it might contain exchangeable image file format (Exif) metadata that includes the image's orientation. Amazon Rekognition uses this orientation information to perform image correction. The bounding box coordinates are translated to represent object locations after the orientation information in the Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata.</p>  <p>Amazon Rekognition doesn’t perform image correction for images in .png format and .jpeg images without orientation information in the image Exif metadata. The bounding box coordinates aren't translated and represent the object locations before the image is rotated. </p>
                            /// - On failure, responds with [`SdkError<DetectFacesError>`](crate::error::DetectFacesError)
    pub fn detect_faces(&self) -> crate::client::fluent_builders::DetectFaces {
                                crate::client::fluent_builders::DetectFaces::new(self.handle.clone())
                            }
}

