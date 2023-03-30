// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`ActivateGateway`](crate::client::fluent_builders::ActivateGateway) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`activation_key(impl Into<String>)`](crate::client::fluent_builders::ActivateGateway::activation_key) / [`set_activation_key(Option<String>)`](crate::client::fluent_builders::ActivateGateway::set_activation_key): <p>Your gateway activation key. You can obtain the activation key by sending an HTTP GET request with redirects enabled to the gateway IP address (port 80). The redirect URL returned in the response provides you the activation key for your gateway in the query string parameter <code>activationKey</code>. It may also include other activation-related parameters, however, these are merely defaults -- the arguments you pass to the <code>ActivateGateway</code> API call determine the actual configuration of your gateway.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/get-activation-key.html">Getting activation key</a> in the <i>Storage Gateway User Guide</i>.</p>
    ///   - [`gateway_name(impl Into<String>)`](crate::client::fluent_builders::ActivateGateway::gateway_name) / [`set_gateway_name(Option<String>)`](crate::client::fluent_builders::ActivateGateway::set_gateway_name): <p>The name you configured for your gateway.</p>
    ///   - [`gateway_timezone(impl Into<String>)`](crate::client::fluent_builders::ActivateGateway::gateway_timezone) / [`set_gateway_timezone(Option<String>)`](crate::client::fluent_builders::ActivateGateway::set_gateway_timezone): <p>A value that indicates the time zone you want to set for the gateway. The time zone is of the format "GMT-hr:mm" or "GMT+hr:mm". For example, GMT-4:00 indicates the time is 4 hours behind GMT. GMT+2:00 indicates the time is 2 hours ahead of GMT. The time zone is used, for example, for scheduling snapshots and your gateway's maintenance schedule.</p>
    ///   - [`gateway_region(impl Into<String>)`](crate::client::fluent_builders::ActivateGateway::gateway_region) / [`set_gateway_region(Option<String>)`](crate::client::fluent_builders::ActivateGateway::set_gateway_region): <p>A value that indicates the Amazon Web Services Region where you want to store your data. The gateway Amazon Web Services Region specified must be the same Amazon Web Services Region as the Amazon Web Services Region in your <code>Host</code> header in the request. For more information about available Amazon Web Services Regions and endpoints for Storage Gateway, see <a href="https://docs.aws.amazon.com/general/latest/gr/sg.html"> Storage Gateway endpoints and quotas</a> in the <i>Amazon Web Services General Reference</i>.</p>  <p>Valid Values: See <a href="https://docs.aws.amazon.com/general/latest/gr/sg.html"> Storage Gateway endpoints and quotas</a> in the <i>Amazon Web Services General Reference</i>. </p>
    ///   - [`gateway_type(impl Into<String>)`](crate::client::fluent_builders::ActivateGateway::gateway_type) / [`set_gateway_type(Option<String>)`](crate::client::fluent_builders::ActivateGateway::set_gateway_type): <p>A value that defines the type of gateway to activate. The type specified is critical to all later functions of the gateway and cannot be changed after activation. The default value is <code>CACHED</code>.</p>  <p>Valid Values: <code>STORED</code> | <code>CACHED</code> | <code>VTL</code> | <code>VTL_SNOW</code> | <code>FILE_S3</code> | <code>FILE_FSX_SMB</code> </p>
    ///   - [`tape_drive_type(impl Into<String>)`](crate::client::fluent_builders::ActivateGateway::tape_drive_type) / [`set_tape_drive_type(Option<String>)`](crate::client::fluent_builders::ActivateGateway::set_tape_drive_type): <p>The value that indicates the type of tape drive to use for tape gateway. This field is optional.</p>  <p>Valid Values: <code>IBM-ULT3580-TD5</code> </p>
    ///   - [`medium_changer_type(impl Into<String>)`](crate::client::fluent_builders::ActivateGateway::medium_changer_type) / [`set_medium_changer_type(Option<String>)`](crate::client::fluent_builders::ActivateGateway::set_medium_changer_type): <p>The value that indicates the type of medium changer to use for tape gateway. This field is optional.</p>  <p>Valid Values: <code>STK-L700</code> | <code>AWS-Gateway-VTL</code> | <code>IBM-03584L32-0402</code> </p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::ActivateGateway::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::ActivateGateway::set_tags): <p>A list of up to 50 tags that you can assign to the gateway. Each tag is a key-value pair.</p> <note>   <p>Valid characters for key and value are letters, spaces, and numbers that can be represented in UTF-8 format, and the following special characters: + - = . _ : / @. The maximum length of a tag's key is 128 characters, and the maximum length for a tag's value is 256 characters.</p>  </note>
                            /// - On success, responds with [`ActivateGatewayOutput`](crate::output::ActivateGatewayOutput) with field(s):
    ///   - [`gateway_arn(Option<String>)`](crate::output::ActivateGatewayOutput::gateway_arn): <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
                            /// - On failure, responds with [`SdkError<ActivateGatewayError>`](crate::error::ActivateGatewayError)
    pub fn activate_gateway(&self) -> crate::client::fluent_builders::ActivateGateway {
                                crate::client::fluent_builders::ActivateGateway::new(self.handle.clone())
                            }
}

