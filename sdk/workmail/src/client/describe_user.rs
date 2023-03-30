// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribeUser`](crate::client::fluent_builders::DescribeUser) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`organization_id(impl Into<String>)`](crate::client::fluent_builders::DescribeUser::organization_id) / [`set_organization_id(Option<String>)`](crate::client::fluent_builders::DescribeUser::set_organization_id): <p>The identifier for the organization under which the user exists.</p>
    ///   - [`user_id(impl Into<String>)`](crate::client::fluent_builders::DescribeUser::user_id) / [`set_user_id(Option<String>)`](crate::client::fluent_builders::DescribeUser::set_user_id): <p>The identifier for the user to be described.</p>
                            /// - On success, responds with [`DescribeUserOutput`](crate::output::DescribeUserOutput) with field(s):
    ///   - [`user_id(Option<String>)`](crate::output::DescribeUserOutput::user_id): <p>The identifier for the described user.</p>
    ///   - [`name(Option<String>)`](crate::output::DescribeUserOutput::name): <p>The name for the user.</p>
    ///   - [`email(Option<String>)`](crate::output::DescribeUserOutput::email): <p>The email of the user.</p>
    ///   - [`display_name(Option<String>)`](crate::output::DescribeUserOutput::display_name): <p>The display name of the user.</p>
    ///   - [`state(Option<EntityState>)`](crate::output::DescribeUserOutput::state): <p>The state of a user: enabled (registered to WorkMail) or disabled (deregistered or never registered to WorkMail).</p>
    ///   - [`user_role(Option<UserRole>)`](crate::output::DescribeUserOutput::user_role): <p>In certain cases, other entities are modeled as users. If interoperability is enabled, resources are imported into WorkMail as users. Because different WorkMail organizations rely on different directory types, administrators can distinguish between an unregistered user (account is disabled and has a user role) and the directory administrators. The values are USER, RESOURCE, and SYSTEM_USER.</p>
    ///   - [`enabled_date(Option<DateTime>)`](crate::output::DescribeUserOutput::enabled_date): <p>The date and time at which the user was enabled for WorkMailusage, in UNIX epoch time format.</p>
    ///   - [`disabled_date(Option<DateTime>)`](crate::output::DescribeUserOutput::disabled_date): <p>The date and time at which the user was disabled for WorkMail usage, in UNIX epoch time format.</p>
                            /// - On failure, responds with [`SdkError<DescribeUserError>`](crate::error::DescribeUserError)
    pub fn describe_user(&self) -> crate::client::fluent_builders::DescribeUser {
                                crate::client::fluent_builders::DescribeUser::new(self.handle.clone())
                            }
}

