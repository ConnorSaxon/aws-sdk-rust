// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateEntitlement`](crate::client::fluent_builders::CreateEntitlement) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateEntitlement::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateEntitlement::set_name): <p>The name of the entitlement.</p>
    ///   - [`stack_name(impl Into<String>)`](crate::client::fluent_builders::CreateEntitlement::stack_name) / [`set_stack_name(Option<String>)`](crate::client::fluent_builders::CreateEntitlement::set_stack_name): <p>The name of the stack with which the entitlement is associated.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateEntitlement::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateEntitlement::set_description): <p>The description of the entitlement.</p>
    ///   - [`app_visibility(AppVisibility)`](crate::client::fluent_builders::CreateEntitlement::app_visibility) / [`set_app_visibility(Option<AppVisibility>)`](crate::client::fluent_builders::CreateEntitlement::set_app_visibility): <p>Specifies whether all or selected apps are entitled.</p>
    ///   - [`attributes(Vec<EntitlementAttribute>)`](crate::client::fluent_builders::CreateEntitlement::attributes) / [`set_attributes(Option<Vec<EntitlementAttribute>>)`](crate::client::fluent_builders::CreateEntitlement::set_attributes): <p>The attributes of the entitlement.</p>
                            /// - On success, responds with [`CreateEntitlementOutput`](crate::output::CreateEntitlementOutput) with field(s):
    ///   - [`entitlement(Option<Entitlement>)`](crate::output::CreateEntitlementOutput::entitlement): <p>The entitlement.</p>
                            /// - On failure, responds with [`SdkError<CreateEntitlementError>`](crate::error::CreateEntitlementError)
    pub fn create_entitlement(&self) -> crate::client::fluent_builders::CreateEntitlement {
                                crate::client::fluent_builders::CreateEntitlement::new(self.handle.clone())
                            }
}

