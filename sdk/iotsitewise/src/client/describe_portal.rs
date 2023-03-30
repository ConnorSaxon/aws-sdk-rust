// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`DescribePortal`](crate::client::fluent_builders::DescribePortal) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`portal_id(impl Into<String>)`](crate::client::fluent_builders::DescribePortal::portal_id) / [`set_portal_id(Option<String>)`](crate::client::fluent_builders::DescribePortal::set_portal_id): <p>The ID of the portal.</p>
                            /// - On success, responds with [`DescribePortalOutput`](crate::output::DescribePortalOutput) with field(s):
    ///   - [`portal_id(Option<String>)`](crate::output::DescribePortalOutput::portal_id): <p>The ID of the portal.</p>
    ///   - [`portal_arn(Option<String>)`](crate::output::DescribePortalOutput::portal_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the portal, which has the following format.</p>  <p> <code>arn:${Partition}:iotsitewise:${Region}:${Account}:portal/${PortalId}</code> </p>
    ///   - [`portal_name(Option<String>)`](crate::output::DescribePortalOutput::portal_name): <p>The name of the portal.</p>
    ///   - [`portal_description(Option<String>)`](crate::output::DescribePortalOutput::portal_description): <p>The portal's description.</p>
    ///   - [`portal_client_id(Option<String>)`](crate::output::DescribePortalOutput::portal_client_id): <p>The IAM Identity Center application generated client ID (used with IAM Identity Center APIs). IoT SiteWise includes <code>portalClientId</code> for only portals that use IAM Identity Center to authenticate users.</p>
    ///   - [`portal_start_url(Option<String>)`](crate::output::DescribePortalOutput::portal_start_url): <p>The URL for the IoT SiteWise Monitor portal. You can use this URL to access portals that use IAM Identity Center for authentication. For portals that use IAM for authentication, you must use the IoT SiteWise console to get a URL that you can use to access the portal.</p>
    ///   - [`portal_contact_email(Option<String>)`](crate::output::DescribePortalOutput::portal_contact_email): <p>The Amazon Web Services administrator's contact email address.</p>
    ///   - [`portal_status(Option<PortalStatus>)`](crate::output::DescribePortalOutput::portal_status): <p>The current status of the portal, which contains a state and any error message.</p>
    ///   - [`portal_creation_date(Option<DateTime>)`](crate::output::DescribePortalOutput::portal_creation_date): <p>The date the portal was created, in Unix epoch time.</p>
    ///   - [`portal_last_update_date(Option<DateTime>)`](crate::output::DescribePortalOutput::portal_last_update_date): <p>The date the portal was last updated, in Unix epoch time.</p>
    ///   - [`portal_logo_image_location(Option<ImageLocation>)`](crate::output::DescribePortalOutput::portal_logo_image_location): <p>The portal's logo image, which is available at a URL.</p>
    ///   - [`role_arn(Option<String>)`](crate::output::DescribePortalOutput::role_arn): <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the service role that allows the portal's users to access your IoT SiteWise resources on your behalf. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/monitor-service-role.html">Using service roles for IoT SiteWise Monitor</a> in the <i>IoT SiteWise User Guide</i>.</p>
    ///   - [`portal_auth_mode(Option<AuthMode>)`](crate::output::DescribePortalOutput::portal_auth_mode): <p>The service to use to authenticate users to the portal.</p>
    ///   - [`notification_sender_email(Option<String>)`](crate::output::DescribePortalOutput::notification_sender_email): <p>The email address that sends alarm notifications.</p>
    ///   - [`alarms(Option<Alarms>)`](crate::output::DescribePortalOutput::alarms): <p>Contains the configuration information of an alarm created in an IoT SiteWise Monitor portal.</p>
                            /// - On failure, responds with [`SdkError<DescribePortalError>`](crate::error::DescribePortalError)
    pub fn describe_portal(&self) -> crate::client::fluent_builders::DescribePortal {
                                crate::client::fluent_builders::DescribePortal::new(self.handle.clone())
                            }
}

