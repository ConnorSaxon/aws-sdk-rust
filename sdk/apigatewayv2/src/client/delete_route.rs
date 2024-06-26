// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRoute`](crate::operation::delete_route::builders::DeleteRouteFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`api_id(impl Into<String>)`](crate::operation::delete_route::builders::DeleteRouteFluentBuilder::api_id) / [`set_api_id(Option<String>)`](crate::operation::delete_route::builders::DeleteRouteFluentBuilder::set_api_id):<br>required: **true**<br><p>The API identifier.</p><br>
    ///   - [`route_id(impl Into<String>)`](crate::operation::delete_route::builders::DeleteRouteFluentBuilder::route_id) / [`set_route_id(Option<String>)`](crate::operation::delete_route::builders::DeleteRouteFluentBuilder::set_route_id):<br>required: **true**<br><p>The route ID.</p><br>
    /// - On success, responds with [`DeleteRouteOutput`](crate::operation::delete_route::DeleteRouteOutput)
    /// - On failure, responds with [`SdkError<DeleteRouteError>`](crate::operation::delete_route::DeleteRouteError)
    pub fn delete_route(&self) -> crate::operation::delete_route::builders::DeleteRouteFluentBuilder {
        crate::operation::delete_route::builders::DeleteRouteFluentBuilder::new(self.handle.clone())
    }
}
