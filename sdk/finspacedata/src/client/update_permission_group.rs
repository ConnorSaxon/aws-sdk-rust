// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`UpdatePermissionGroup`](crate::client::fluent_builders::UpdatePermissionGroup) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`permission_group_id(impl Into<String>)`](crate::client::fluent_builders::UpdatePermissionGroup::permission_group_id) / [`set_permission_group_id(Option<String>)`](crate::client::fluent_builders::UpdatePermissionGroup::set_permission_group_id): <p>The unique identifier for the permission group to update.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::UpdatePermissionGroup::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::UpdatePermissionGroup::set_name): <p>The name of the permission group.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::UpdatePermissionGroup::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::UpdatePermissionGroup::set_description): <p>A brief description for the permission group.</p>
    ///   - [`application_permissions(Vec<ApplicationPermission>)`](crate::client::fluent_builders::UpdatePermissionGroup::application_permissions) / [`set_application_permissions(Option<Vec<ApplicationPermission>>)`](crate::client::fluent_builders::UpdatePermissionGroup::set_application_permissions): <p>The permissions that are granted to a specific group for accessing the FinSpace application.</p> <important>   <p>When assigning application permissions, be aware that the permission <code>ManageUsersAndGroups</code> allows users to grant themselves or others access to any functionality in their FinSpace environment's application. It should only be granted to trusted users.</p>  </important>  <ul>   <li> <p> <code>CreateDataset</code> – Group members can create new datasets.</p> </li>   <li> <p> <code>ManageClusters</code> – Group members can manage Apache Spark clusters from FinSpace notebooks.</p> </li>   <li> <p> <code>ManageUsersAndGroups</code> – Group members can manage users and permission groups. This is a privileged permission that allows users to grant themselves or others access to any functionality in the application. It should only be granted to trusted users.</p> </li>   <li> <p> <code>ManageAttributeSets</code> – Group members can manage attribute sets.</p> </li>   <li> <p> <code>ViewAuditData</code> – Group members can view audit data.</p> </li>   <li> <p> <code>AccessNotebooks</code> – Group members will have access to FinSpace notebooks.</p> </li>   <li> <p> <code>GetTemporaryCredentials</code> – Group members can get temporary API credentials.</p> </li>  </ul>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::UpdatePermissionGroup::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::UpdatePermissionGroup::set_client_token): <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
                            /// - On success, responds with [`UpdatePermissionGroupOutput`](crate::output::UpdatePermissionGroupOutput) with field(s):
    ///   - [`permission_group_id(Option<String>)`](crate::output::UpdatePermissionGroupOutput::permission_group_id): <p>The unique identifier for the updated permission group.</p>
                            /// - On failure, responds with [`SdkError<UpdatePermissionGroupError>`](crate::error::UpdatePermissionGroupError)
    pub fn update_permission_group(&self) -> crate::client::fluent_builders::UpdatePermissionGroup {
                                crate::client::fluent_builders::UpdatePermissionGroup::new(self.handle.clone())
                            }
}

