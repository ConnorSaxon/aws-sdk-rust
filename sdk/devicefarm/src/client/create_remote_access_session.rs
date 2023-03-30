// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateRemoteAccessSession`](crate::client::fluent_builders::CreateRemoteAccessSession) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`project_arn(impl Into<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::project_arn) / [`set_project_arn(Option<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::set_project_arn): <p>The Amazon Resource Name (ARN) of the project for which you want to create a remote access session.</p>
    ///   - [`device_arn(impl Into<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::device_arn) / [`set_device_arn(Option<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::set_device_arn): <p>The ARN of the device for which you want to create a remote access session.</p>
    ///   - [`instance_arn(impl Into<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::instance_arn) / [`set_instance_arn(Option<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::set_instance_arn): <p>The Amazon Resource Name (ARN) of the device instance for which you want to create a remote access session.</p>
    ///   - [`ssh_public_key(impl Into<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::ssh_public_key) / [`set_ssh_public_key(Option<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::set_ssh_public_key): <p>Ignored. The public key of the <code>ssh</code> key pair you want to use for connecting to remote devices in your remote debugging session. This key is required only if <code>remoteDebugEnabled</code> is set to <code>true</code>.</p>  <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>.</p>
    ///   - [`remote_debug_enabled(bool)`](crate::client::fluent_builders::CreateRemoteAccessSession::remote_debug_enabled) / [`set_remote_debug_enabled(Option<bool>)`](crate::client::fluent_builders::CreateRemoteAccessSession::set_remote_debug_enabled): <p>Set to <code>true</code> if you want to access devices remotely for debugging in your remote access session.</p>  <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>.</p>
    ///   - [`remote_record_enabled(bool)`](crate::client::fluent_builders::CreateRemoteAccessSession::remote_record_enabled) / [`set_remote_record_enabled(Option<bool>)`](crate::client::fluent_builders::CreateRemoteAccessSession::set_remote_record_enabled): <p>Set to <code>true</code> to enable remote recording for the remote access session.</p>
    ///   - [`remote_record_app_arn(impl Into<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::remote_record_app_arn) / [`set_remote_record_app_arn(Option<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::set_remote_record_app_arn): <p>The Amazon Resource Name (ARN) for the app to be recorded in the remote access session.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::set_name): <p>The name of the remote access session to create.</p>
    ///   - [`client_id(impl Into<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::client_id) / [`set_client_id(Option<String>)`](crate::client::fluent_builders::CreateRemoteAccessSession::set_client_id): <p>Unique identifier for the client. If you want access to multiple devices on the same client, you should pass the same <code>clientId</code> value in each call to <code>CreateRemoteAccessSession</code>. This identifier is required only if <code>remoteDebugEnabled</code> is set to <code>true</code>.</p>  <p>Remote debugging is <a href="https://docs.aws.amazon.com/devicefarm/latest/developerguide/history.html">no longer supported</a>.</p>
    ///   - [`configuration(CreateRemoteAccessSessionConfiguration)`](crate::client::fluent_builders::CreateRemoteAccessSession::configuration) / [`set_configuration(Option<CreateRemoteAccessSessionConfiguration>)`](crate::client::fluent_builders::CreateRemoteAccessSession::set_configuration): <p>The configuration information for the remote access session request.</p>
    ///   - [`interaction_mode(InteractionMode)`](crate::client::fluent_builders::CreateRemoteAccessSession::interaction_mode) / [`set_interaction_mode(Option<InteractionMode>)`](crate::client::fluent_builders::CreateRemoteAccessSession::set_interaction_mode): <p>The interaction mode of the remote access session. Valid values are:</p>  <ul>   <li> <p>INTERACTIVE: You can interact with the iOS device by viewing, touching, and rotating the screen. You cannot run XCUITest framework-based tests in this mode.</p> </li>   <li> <p>NO_VIDEO: You are connected to the device, but cannot interact with it or view the screen. This mode has the fastest test execution speed. You can run XCUITest framework-based tests in this mode.</p> </li>   <li> <p>VIDEO_ONLY: You can view the screen, but cannot touch or rotate it. You can run XCUITest framework-based tests and watch the screen in this mode.</p> </li>  </ul>
    ///   - [`skip_app_resign(bool)`](crate::client::fluent_builders::CreateRemoteAccessSession::skip_app_resign) / [`set_skip_app_resign(Option<bool>)`](crate::client::fluent_builders::CreateRemoteAccessSession::set_skip_app_resign): <p>When set to <code>true</code>, for private devices, Device Farm does not sign your app again. For public devices, Device Farm always signs your apps again.</p>  <p>For more information on how Device Farm modifies your uploads during tests, see <a href="http://aws.amazon.com/device-farm/faqs/">Do you modify my app?</a> </p>
                            /// - On success, responds with [`CreateRemoteAccessSessionOutput`](crate::output::CreateRemoteAccessSessionOutput) with field(s):
    ///   - [`remote_access_session(Option<RemoteAccessSession>)`](crate::output::CreateRemoteAccessSessionOutput::remote_access_session): <p>A container that describes the remote access session when the request to create a remote access session is sent.</p>
                            /// - On failure, responds with [`SdkError<CreateRemoteAccessSessionError>`](crate::error::CreateRemoteAccessSessionError)
    pub fn create_remote_access_session(&self) -> crate::client::fluent_builders::CreateRemoteAccessSession {
                                crate::client::fluent_builders::CreateRemoteAccessSession::new(self.handle.clone())
                            }
}

