// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DetectProtectiveEquipment`](crate::client::fluent_builders::DetectProtectiveEquipment) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`image(Image)`](crate::client::fluent_builders::DetectProtectiveEquipment::image) / [`set_image(Option<Image>)`](crate::client::fluent_builders::DetectProtectiveEquipment::set_image): <p>The image in which you want to detect PPE on detected persons. The image can be passed as image bytes or you can reference an image stored in an Amazon S3 bucket. </p>
    ///   - [`summarization_attributes(ProtectiveEquipmentSummarizationAttributes)`](crate::client::fluent_builders::DetectProtectiveEquipment::summarization_attributes) / [`set_summarization_attributes(Option<ProtectiveEquipmentSummarizationAttributes>)`](crate::client::fluent_builders::DetectProtectiveEquipment::set_summarization_attributes): <p>An array of PPE types that you want to summarize.</p>
                            /// - On success, responds with [`DetectProtectiveEquipmentOutput`](crate::output::DetectProtectiveEquipmentOutput) with field(s):
    ///   - [`protective_equipment_model_version(Option<String>)`](crate::output::DetectProtectiveEquipmentOutput::protective_equipment_model_version): <p>The version number of the PPE detection model used to detect PPE in the image.</p>
    ///   - [`persons(Option<Vec<ProtectiveEquipmentPerson>>)`](crate::output::DetectProtectiveEquipmentOutput::persons): <p>An array of persons detected in the image (including persons not wearing PPE).</p>
    ///   - [`summary(Option<ProtectiveEquipmentSummary>)`](crate::output::DetectProtectiveEquipmentOutput::summary): <p>Summary information for the types of PPE specified in the <code>SummarizationAttributes</code> input parameter.</p>
                            /// - On failure, responds with [`SdkError<DetectProtectiveEquipmentError>`](crate::error::DetectProtectiveEquipmentError)
    pub fn detect_protective_equipment(&self) -> crate::client::fluent_builders::DetectProtectiveEquipment {
                                crate::client::fluent_builders::DetectProtectiveEquipment::new(self.handle.clone())
                            }
}

