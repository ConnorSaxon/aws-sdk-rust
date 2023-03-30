// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`AdminListDevices`](crate::client::fluent_builders::AdminListDevices) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`user_pool_id(impl Into<String>)`](crate::client::fluent_builders::AdminListDevices::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::client::fluent_builders::AdminListDevices::set_user_pool_id): <p>The user pool ID.</p>
    ///   - [`username(impl Into<String>)`](crate::client::fluent_builders::AdminListDevices::username) / [`set_username(Option<String>)`](crate::client::fluent_builders::AdminListDevices::set_username): <p>The user name.</p>
    ///   - [`limit(i32)`](crate::client::fluent_builders::AdminListDevices::limit) / [`set_limit(Option<i32>)`](crate::client::fluent_builders::AdminListDevices::set_limit): <p>The limit of the devices request.</p>
    ///   - [`pagination_token(impl Into<String>)`](crate::client::fluent_builders::AdminListDevices::pagination_token) / [`set_pagination_token(Option<String>)`](crate::client::fluent_builders::AdminListDevices::set_pagination_token): <p>The pagination token.</p>
                            /// - On success, responds with [`AdminListDevicesOutput`](crate::output::AdminListDevicesOutput) with field(s):
    ///   - [`devices(Option<Vec<DeviceType>>)`](crate::output::AdminListDevicesOutput::devices): <p>The devices in the list of devices response.</p>
    ///   - [`pagination_token(Option<String>)`](crate::output::AdminListDevicesOutput::pagination_token): <p>The pagination token.</p>
                            /// - On failure, responds with [`SdkError<AdminListDevicesError>`](crate::error::AdminListDevicesError)
    pub fn admin_list_devices(&self) -> crate::client::fluent_builders::AdminListDevices {
                                crate::client::fluent_builders::AdminListDevices::new(self.handle.clone())
                            }
}

