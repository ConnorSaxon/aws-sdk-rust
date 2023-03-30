// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_gateway_routes_output_next_token(input: &crate::output::ListGatewayRoutesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_meshes_output_next_token(input: &crate::output::ListMeshesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_routes_output_next_token(input: &crate::output::ListRoutesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_tags_for_resource_output_next_token(input: &crate::output::ListTagsForResourceOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_virtual_gateways_output_next_token(input: &crate::output::ListVirtualGatewaysOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_virtual_nodes_output_next_token(input: &crate::output::ListVirtualNodesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_virtual_routers_output_next_token(input: &crate::output::ListVirtualRoutersOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_list_virtual_services_output_next_token(input: &crate::output::ListVirtualServicesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_gateway_routes_output_gateway_routes(input: crate::output::ListGatewayRoutesOutput) -> std::option::Option<std::vec::Vec<crate::model::GatewayRouteRef>> {
                    let input = match input.gateway_routes {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_meshes_output_meshes(input: crate::output::ListMeshesOutput) -> std::option::Option<std::vec::Vec<crate::model::MeshRef>> {
                    let input = match input.meshes {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_routes_output_routes(input: crate::output::ListRoutesOutput) -> std::option::Option<std::vec::Vec<crate::model::RouteRef>> {
                    let input = match input.routes {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_tags_for_resource_output_tags(input: crate::output::ListTagsForResourceOutput) -> std::option::Option<std::vec::Vec<crate::model::TagRef>> {
                    let input = match input.tags {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_virtual_gateways_output_virtual_gateways(input: crate::output::ListVirtualGatewaysOutput) -> std::option::Option<std::vec::Vec<crate::model::VirtualGatewayRef>> {
                    let input = match input.virtual_gateways {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_virtual_nodes_output_virtual_nodes(input: crate::output::ListVirtualNodesOutput) -> std::option::Option<std::vec::Vec<crate::model::VirtualNodeRef>> {
                    let input = match input.virtual_nodes {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_virtual_routers_output_virtual_routers(input: crate::output::ListVirtualRoutersOutput) -> std::option::Option<std::vec::Vec<crate::model::VirtualRouterRef>> {
                    let input = match input.virtual_routers {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_list_virtual_services_output_virtual_services(input: crate::output::ListVirtualServicesOutput) -> std::option::Option<std::vec::Vec<crate::model::VirtualServiceRef>> {
                    let input = match input.virtual_services {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

