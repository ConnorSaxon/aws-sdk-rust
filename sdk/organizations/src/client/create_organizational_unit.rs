// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateOrganizationalUnit`](crate::client::fluent_builders::CreateOrganizationalUnit) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`parent_id(impl Into<String>)`](crate::client::fluent_builders::CreateOrganizationalUnit::parent_id) / [`set_parent_id(Option<String>)`](crate::client::fluent_builders::CreateOrganizationalUnit::set_parent_id): <p>The unique identifier (ID) of the parent root or OU that you want to create the new OU in.</p>  <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> for a parent ID string requires one of the following:</p>  <ul>   <li> <p> <b>Root</b> - A string that begins with "r-" followed by from 4 to 32 lowercase letters or digits.</p> </li>   <li> <p> <b>Organizational unit (OU)</b> - A string that begins with "ou-" followed by from 4 to 32 lowercase letters or digits (the ID of the root that the OU is in). This string is followed by a second "-" dash and from 8 to 32 additional lowercase letters or digits.</p> </li>  </ul>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateOrganizationalUnit::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateOrganizationalUnit::set_name): <p>The friendly name to assign to the new OU.</p>
    ///   - [`tags(Vec<Tag>)`](crate::client::fluent_builders::CreateOrganizationalUnit::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::client::fluent_builders::CreateOrganizationalUnit::set_tags): <p>A list of tags that you want to attach to the newly created OU. For each tag in the list, you must specify both a tag key and a value. You can set the value to an empty string, but you can't set it to <code>null</code>. For more information about tagging, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_tagging.html">Tagging Organizations resources</a> in the Organizations User Guide.</p> <note>   <p>If any one of the tags is invalid or if you exceed the allowed number of tags for an OU, then the entire request fails and the OU is not created.</p>  </note>
                            /// - On success, responds with [`CreateOrganizationalUnitOutput`](crate::output::CreateOrganizationalUnitOutput) with field(s):
    ///   - [`organizational_unit(Option<OrganizationalUnit>)`](crate::output::CreateOrganizationalUnitOutput::organizational_unit): <p>A structure that contains details about the newly created OU.</p>
                            /// - On failure, responds with [`SdkError<CreateOrganizationalUnitError>`](crate::error::CreateOrganizationalUnitError)
    pub fn create_organizational_unit(&self) -> crate::client::fluent_builders::CreateOrganizationalUnit {
                                crate::client::fluent_builders::CreateOrganizationalUnit::new(self.handle.clone())
                            }
}

