// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_route_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::RouteData>, crate::error::DescribeRouteError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_route_data::de_route_data_payload(body).map_err(crate::error::DescribeRouteError::unhandled)
    }).transpose()
}

