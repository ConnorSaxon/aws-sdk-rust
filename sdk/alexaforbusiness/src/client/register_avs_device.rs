// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RegisterAVSDevice`](crate::client::fluent_builders::RegisterAVSDevice) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`client_id(impl Into<String>)`](crate::client::fluent_builders::RegisterAVSDevice::client_id) / [`set_client_id(Option<String>)`](crate::client::fluent_builders::RegisterAVSDevice::set_client_id): <p>The client ID of the OEM used for code-based linking authorization on an AVS device.</p>
    ///   - [`user_code(impl Into<String>)`](crate::client::fluent_builders::RegisterAVSDevice::user_code) / [`set_user_code(Option<String>)`](crate::client::fluent_builders::RegisterAVSDevice::set_user_code): <p>The code that is obtained after your AVS device has made a POST request to LWA as a part of the Device Authorization Request component of the OAuth code-based linking specification.</p>
    ///   - [`product_id(impl Into<String>)`](crate::client::fluent_builders::RegisterAVSDevice::product_id) / [`set_product_id(Option<String>)`](crate::client::fluent_builders::RegisterAVSDevice::set_product_id): <p>The product ID used to identify your AVS device during authorization.</p>
    ///   - [`device_serial_number(impl Into<String>)`](crate::client::fluent_builders::RegisterAVSDevice::device_serial_number) / [`set_device_serial_number(Option<String>)`](crate::client::fluent_builders::RegisterAVSDevice::set_device_serial_number): <p>The key generated by the OEM that uniquely identifies a specified instance of your AVS device.</p>
    ///   - [`amazon_id(impl Into<String>)`](crate::client::fluent_builders::RegisterAVSDevice::amazon_id) / [`set_amazon_id(Option<String>)`](crate::client::fluent_builders::RegisterAVSDevice::set_amazon_id): <p>The device type ID for your AVS device generated by Amazon when the OEM creates a new product on Amazon's Developer Console.</p>
    ///   - [`room_arn(impl Into<String>)`](crate::client::fluent_builders::RegisterAVSDevice::room_arn) / [`set_room_arn(Option<String>)`](crate::client::fluent_builders::RegisterAVSDevice::set_room_arn): <p>The Amazon Resource Name (ARN) of the room with which to associate your AVS device.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::RegisterAVSDevice::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::RegisterAVSDevice::set_tags): <p>The tags to be added to the specified resource. Do not provide system tags.</p>
                            /// - On success, responds with [`RegisterAvsDeviceOutput`](crate::output::RegisterAvsDeviceOutput) with field(s):
    ///   - [`device_arn(Option<String>)`](crate::output::RegisterAvsDeviceOutput::device_arn): <p>The ARN of the device.</p>
                            /// - On failure, responds with [`SdkError<RegisterAVSDeviceError>`](crate::error::RegisterAVSDeviceError)
    pub fn register_avs_device(&self) -> crate::client::fluent_builders::RegisterAVSDevice {
                                crate::client::fluent_builders::RegisterAVSDevice::new(self.handle.clone())
                            }
}

