// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateVehicle`](crate::client::fluent_builders::CreateVehicle) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`vehicle_name(impl Into<String>)`](crate::client::fluent_builders::CreateVehicle::vehicle_name) / [`set_vehicle_name(Option<String>)`](crate::client::fluent_builders::CreateVehicle::set_vehicle_name): <p> The unique ID of the vehicle to create. </p>
    ///   - [`model_manifest_arn(impl Into<String>)`](crate::client::fluent_builders::CreateVehicle::model_manifest_arn) / [`set_model_manifest_arn(Option<String>)`](crate::client::fluent_builders::CreateVehicle::set_model_manifest_arn): <p> The Amazon Resource Name ARN of a vehicle model. </p>
    ///   - [`decoder_manifest_arn(impl Into<String>)`](crate::client::fluent_builders::CreateVehicle::decoder_manifest_arn) / [`set_decoder_manifest_arn(Option<String>)`](crate::client::fluent_builders::CreateVehicle::set_decoder_manifest_arn): <p> The ARN of a decoder manifest. </p>
    ///   - [`attributes(HashMap<String, String>)`](crate::client::fluent_builders::CreateVehicle::attributes) / [`set_attributes(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateVehicle::set_attributes): <p>Static information about a vehicle in a key-value pair. For example: <code>"engineType"</code> : <code>"1.3 L R2"</code> </p>
    ///   - [`association_behavior(VehicleAssociationBehavior)`](crate::client::fluent_builders::CreateVehicle::association_behavior) / [`set_association_behavior(Option<VehicleAssociationBehavior>)`](crate::client::fluent_builders::CreateVehicle::set_association_behavior): <p> An option to create a new Amazon Web Services IoT thing when creating a vehicle, or to validate an existing Amazon Web Services IoT thing as a vehicle. </p>  <p>Default: <code></code> </p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateVehicle::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateVehicle::set_tags): <p>Metadata that can be used to manage the vehicle.</p>
                            /// - On success, responds with [`CreateVehicleOutput`](crate::output::CreateVehicleOutput) with field(s):
    ///   - [`vehicle_name(Option<String>)`](crate::output::CreateVehicleOutput::vehicle_name): <p>The unique ID of the created vehicle.</p>
    ///   - [`arn(Option<String>)`](crate::output::CreateVehicleOutput::arn): <p> The ARN of the created vehicle. </p>
    ///   - [`thing_arn(Option<String>)`](crate::output::CreateVehicleOutput::thing_arn): <p> The ARN of a created or validated Amazon Web Services IoT thing. </p>
                            /// - On failure, responds with [`SdkError<CreateVehicleError>`](crate::error::CreateVehicleError)
    pub fn create_vehicle(&self) -> crate::client::fluent_builders::CreateVehicle {
                                crate::client::fluent_builders::CreateVehicle::new(self.handle.clone())
                            }
}

