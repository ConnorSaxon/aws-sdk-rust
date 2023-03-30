// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_accept_direct_connect_gateway_association_proposal;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_allocate_connection_on_interconnect;

pub(crate) mod shape_allocate_hosted_connection;

pub(crate) mod shape_allocate_private_virtual_interface;

pub(crate) mod shape_allocate_public_virtual_interface;

pub(crate) mod shape_allocate_transit_virtual_interface;

pub(crate) mod shape_associate_connection_with_lag;

pub(crate) mod shape_associate_hosted_connection;

pub(crate) mod shape_associate_mac_sec_key;

pub(crate) mod shape_associate_virtual_interface;

pub(crate) mod shape_confirm_connection;

pub(crate) mod shape_confirm_customer_agreement;

pub(crate) mod shape_confirm_private_virtual_interface;

pub(crate) mod shape_confirm_public_virtual_interface;

pub(crate) mod shape_confirm_transit_virtual_interface;

pub(crate) mod shape_create_bgp_peer;

pub(crate) mod shape_create_connection;

pub(crate) mod shape_create_direct_connect_gateway;

pub(crate) mod shape_create_direct_connect_gateway_association;

pub(crate) mod shape_create_direct_connect_gateway_association_proposal;

pub(crate) mod shape_create_interconnect;

pub(crate) mod shape_create_lag;

pub(crate) mod shape_create_private_virtual_interface;

pub(crate) mod shape_create_public_virtual_interface;

pub(crate) mod shape_create_transit_virtual_interface;

pub(crate) mod shape_delete_bgp_peer;

pub(crate) mod shape_delete_connection;

pub(crate) mod shape_delete_direct_connect_gateway;

pub(crate) mod shape_delete_direct_connect_gateway_association;

pub(crate) mod shape_delete_direct_connect_gateway_association_proposal;

pub(crate) mod shape_delete_interconnect;

pub(crate) mod shape_delete_lag;

pub(crate) mod shape_delete_virtual_interface;

pub(crate) mod shape_describe_connection_loa;

pub(crate) mod shape_describe_connections;

pub(crate) mod shape_describe_connections_on_interconnect;

pub(crate) mod shape_describe_customer_metadata;

pub(crate) mod shape_describe_direct_connect_gateway_association_proposals;

pub(crate) mod shape_describe_direct_connect_gateway_associations;

pub(crate) mod shape_describe_direct_connect_gateway_attachments;

pub(crate) mod shape_describe_direct_connect_gateways;

pub(crate) mod shape_describe_hosted_connections;

pub(crate) mod shape_describe_interconnect_loa;

pub(crate) mod shape_describe_interconnects;

pub(crate) mod shape_describe_lags;

pub(crate) mod shape_describe_loa;

pub(crate) mod shape_describe_locations;

pub(crate) mod shape_describe_router_configuration;

pub(crate) mod shape_describe_tags;

pub(crate) mod shape_describe_virtual_gateways;

pub(crate) mod shape_describe_virtual_interfaces;

pub(crate) mod shape_disassociate_connection_from_lag;

pub(crate) mod shape_disassociate_mac_sec_key;

pub(crate) mod shape_list_virtual_interface_test_history;

pub(crate) mod shape_start_bgp_failover_test;

pub(crate) mod shape_stop_bgp_failover_test;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_connection;

pub(crate) mod shape_update_direct_connect_gateway;

pub(crate) mod shape_update_direct_connect_gateway_association;

pub(crate) mod shape_update_lag;

pub(crate) mod shape_update_virtual_interface_attributes;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_accept_direct_connect_gateway_association_proposal_input;

pub(crate) mod shape_allocate_connection_on_interconnect_input;

pub(crate) mod shape_allocate_hosted_connection_input;

pub(crate) mod shape_allocate_private_virtual_interface_input;

pub(crate) mod shape_allocate_public_virtual_interface_input;

pub(crate) mod shape_allocate_transit_virtual_interface_input;

pub(crate) mod shape_associate_connection_with_lag_input;

pub(crate) mod shape_associate_hosted_connection_input;

pub(crate) mod shape_associate_mac_sec_key_input;

pub(crate) mod shape_associate_virtual_interface_input;

pub(crate) mod shape_confirm_connection_input;

pub(crate) mod shape_confirm_customer_agreement_input;

pub(crate) mod shape_confirm_private_virtual_interface_input;

pub(crate) mod shape_confirm_public_virtual_interface_input;

pub(crate) mod shape_confirm_transit_virtual_interface_input;

pub(crate) mod shape_create_bgp_peer_input;

pub(crate) mod shape_create_connection_input;

pub(crate) mod shape_create_direct_connect_gateway_association_input;

pub(crate) mod shape_create_direct_connect_gateway_association_proposal_input;

pub(crate) mod shape_create_direct_connect_gateway_input;

pub(crate) mod shape_create_interconnect_input;

pub(crate) mod shape_create_lag_input;

pub(crate) mod shape_create_private_virtual_interface_input;

pub(crate) mod shape_create_public_virtual_interface_input;

pub(crate) mod shape_create_transit_virtual_interface_input;

pub(crate) mod shape_delete_bgp_peer_input;

pub(crate) mod shape_delete_connection_input;

pub(crate) mod shape_delete_direct_connect_gateway_association_input;

pub(crate) mod shape_delete_direct_connect_gateway_association_proposal_input;

pub(crate) mod shape_delete_direct_connect_gateway_input;

pub(crate) mod shape_delete_interconnect_input;

pub(crate) mod shape_delete_lag_input;

pub(crate) mod shape_delete_virtual_interface_input;

pub(crate) mod shape_describe_connection_loa_input;

pub(crate) mod shape_describe_connections_input;

pub(crate) mod shape_describe_connections_on_interconnect_input;

pub(crate) mod shape_describe_direct_connect_gateway_association_proposals_input;

pub(crate) mod shape_describe_direct_connect_gateway_associations_input;

pub(crate) mod shape_describe_direct_connect_gateway_attachments_input;

pub(crate) mod shape_describe_direct_connect_gateways_input;

pub(crate) mod shape_describe_hosted_connections_input;

pub(crate) mod shape_describe_interconnect_loa_input;

pub(crate) mod shape_describe_interconnects_input;

pub(crate) mod shape_describe_lags_input;

pub(crate) mod shape_describe_loa_input;

pub(crate) mod shape_describe_router_configuration_input;

pub(crate) mod shape_describe_tags_input;

pub(crate) mod shape_describe_virtual_interfaces_input;

pub(crate) mod shape_direct_connect_client_exception;

pub(crate) mod shape_direct_connect_server_exception;

pub(crate) mod shape_disassociate_connection_from_lag_input;

pub(crate) mod shape_disassociate_mac_sec_key_input;

pub(crate) mod shape_duplicate_tag_keys_exception;

pub(crate) mod shape_list_virtual_interface_test_history_input;

pub(crate) mod shape_start_bgp_failover_test_input;

pub(crate) mod shape_stop_bgp_failover_test_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_too_many_tags_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_connection_input;

pub(crate) mod shape_update_direct_connect_gateway_association_input;

pub(crate) mod shape_update_direct_connect_gateway_input;

pub(crate) mod shape_update_lag_input;

pub(crate) mod shape_update_virtual_interface_attributes_input;

pub(crate) mod shape_agreement_list;

pub(crate) mod shape_bgp_peer_list;

pub(crate) mod shape_connection_list;

pub(crate) mod shape_direct_connect_gateway;

pub(crate) mod shape_direct_connect_gateway_association;

pub(crate) mod shape_direct_connect_gateway_association_list;

pub(crate) mod shape_direct_connect_gateway_association_proposal;

pub(crate) mod shape_direct_connect_gateway_association_proposal_list;

pub(crate) mod shape_direct_connect_gateway_attachment_list;

pub(crate) mod shape_direct_connect_gateway_list;

pub(crate) mod shape_interconnect_list;

pub(crate) mod shape_lag_list;

pub(crate) mod shape_loa;

pub(crate) mod shape_location_list;

pub(crate) mod shape_mac_sec_key_list;

pub(crate) mod shape_new_bgp_peer;

pub(crate) mod shape_new_private_virtual_interface;

pub(crate) mod shape_new_private_virtual_interface_allocation;

pub(crate) mod shape_new_public_virtual_interface;

pub(crate) mod shape_new_public_virtual_interface_allocation;

pub(crate) mod shape_new_transit_virtual_interface;

pub(crate) mod shape_new_transit_virtual_interface_allocation;

pub(crate) mod shape_resource_tag_list;

pub(crate) mod shape_route_filter_prefix;

pub(crate) mod shape_route_filter_prefix_list;

pub(crate) mod shape_router_type;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_virtual_gateway_list;

pub(crate) mod shape_virtual_interface;

pub(crate) mod shape_virtual_interface_list;

pub(crate) mod shape_virtual_interface_test_history;

pub(crate) mod shape_virtual_interface_test_history_list;

pub(crate) mod shape_associated_gateway;

pub(crate) mod shape_bgp_peer;

pub(crate) mod shape_bgp_peer_id_list;

pub(crate) mod shape_connection;

pub(crate) mod shape_customer_agreement;

pub(crate) mod shape_direct_connect_gateway_attachment;

pub(crate) mod shape_interconnect;

pub(crate) mod shape_lag;

pub(crate) mod shape_location;

pub(crate) mod shape_mac_sec_key;

pub(crate) mod shape_resource_tag;

pub(crate) mod shape_virtual_gateway;

pub(crate) mod shape_available_mac_sec_port_speeds;

pub(crate) mod shape_available_port_speeds;

pub(crate) mod shape_provider_list;

