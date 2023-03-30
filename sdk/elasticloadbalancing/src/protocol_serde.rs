// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    crate::rest_xml_wrapped_errors::parse_error_metadata(response.body().as_ref())
}

pub(crate) mod shape_add_tags;

pub(crate) mod shape_add_tags_input;

pub(crate) mod shape_apply_security_groups_to_load_balancer;

pub(crate) mod shape_apply_security_groups_to_load_balancer_input;

pub(crate) mod shape_attach_load_balancer_to_subnets;

pub(crate) mod shape_attach_load_balancer_to_subnets_input;

pub(crate) mod shape_configure_health_check;

pub(crate) mod shape_configure_health_check_input;

pub(crate) mod shape_create_app_cookie_stickiness_policy;

pub(crate) mod shape_create_app_cookie_stickiness_policy_input;

pub(crate) mod shape_create_lb_cookie_stickiness_policy;

pub(crate) mod shape_create_lb_cookie_stickiness_policy_input;

pub(crate) mod shape_create_load_balancer;

pub(crate) mod shape_create_load_balancer_input;

pub(crate) mod shape_create_load_balancer_listeners;

pub(crate) mod shape_create_load_balancer_listeners_input;

pub(crate) mod shape_create_load_balancer_policy;

pub(crate) mod shape_create_load_balancer_policy_input;

pub(crate) mod shape_delete_load_balancer;

pub(crate) mod shape_delete_load_balancer_input;

pub(crate) mod shape_delete_load_balancer_listeners;

pub(crate) mod shape_delete_load_balancer_listeners_input;

pub(crate) mod shape_delete_load_balancer_policy;

pub(crate) mod shape_delete_load_balancer_policy_input;

pub(crate) mod shape_deregister_instances_from_load_balancer;

pub(crate) mod shape_deregister_instances_from_load_balancer_input;

pub(crate) mod shape_describe_account_limits;

pub(crate) mod shape_describe_account_limits_input;

pub(crate) mod shape_describe_instance_health;

pub(crate) mod shape_describe_instance_health_input;

pub(crate) mod shape_describe_load_balancer_attributes;

pub(crate) mod shape_describe_load_balancer_attributes_input;

pub(crate) mod shape_describe_load_balancer_policies;

pub(crate) mod shape_describe_load_balancer_policies_input;

pub(crate) mod shape_describe_load_balancer_policy_types;

pub(crate) mod shape_describe_load_balancer_policy_types_input;

pub(crate) mod shape_describe_load_balancers;

pub(crate) mod shape_describe_load_balancers_input;

pub(crate) mod shape_describe_tags;

pub(crate) mod shape_describe_tags_input;

pub(crate) mod shape_detach_load_balancer_from_subnets;

pub(crate) mod shape_detach_load_balancer_from_subnets_input;

pub(crate) mod shape_disable_availability_zones_for_load_balancer;

pub(crate) mod shape_disable_availability_zones_for_load_balancer_input;

pub(crate) mod shape_enable_availability_zones_for_load_balancer;

pub(crate) mod shape_enable_availability_zones_for_load_balancer_input;

pub(crate) mod shape_modify_load_balancer_attributes;

pub(crate) mod shape_modify_load_balancer_attributes_input;

pub(crate) mod shape_register_instances_with_load_balancer;

pub(crate) mod shape_register_instances_with_load_balancer_input;

pub(crate) mod shape_remove_tags;

pub(crate) mod shape_remove_tags_input;

pub(crate) mod shape_set_load_balancer_listener_ssl_certificate;

pub(crate) mod shape_set_load_balancer_listener_ssl_certificate_input;

pub(crate) mod shape_set_load_balancer_policies_for_backend_server;

pub(crate) mod shape_set_load_balancer_policies_for_backend_server_input;

pub(crate) mod shape_set_load_balancer_policies_of_listener;

pub(crate) mod shape_set_load_balancer_policies_of_listener_input;

pub(crate) mod shape_access_point_not_found_exception;

pub(crate) mod shape_certificate_not_found_exception;

pub(crate) mod shape_dependency_throttle_exception;

pub(crate) mod shape_duplicate_access_point_name_exception;

pub(crate) mod shape_duplicate_listener_exception;

pub(crate) mod shape_duplicate_policy_name_exception;

pub(crate) mod shape_duplicate_tag_keys_exception;

pub(crate) mod shape_health_check;

pub(crate) mod shape_instance;

pub(crate) mod shape_invalid_configuration_request_exception;

pub(crate) mod shape_invalid_end_point_exception;

pub(crate) mod shape_invalid_scheme_exception;

pub(crate) mod shape_invalid_security_group_exception;

pub(crate) mod shape_invalid_subnet_exception;

pub(crate) mod shape_listener;

pub(crate) mod shape_listener_not_found_exception;

pub(crate) mod shape_load_balancer_attribute_not_found_exception;

pub(crate) mod shape_load_balancer_attributes;

pub(crate) mod shape_operation_not_permitted_exception;

pub(crate) mod shape_policy_attribute;

pub(crate) mod shape_policy_not_found_exception;

pub(crate) mod shape_policy_type_not_found_exception;

pub(crate) mod shape_subnet_not_found_exception;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_key_only;

pub(crate) mod shape_too_many_access_points_exception;

pub(crate) mod shape_too_many_policies_exception;

pub(crate) mod shape_too_many_tags_exception;

pub(crate) mod shape_unsupported_protocol_exception;

pub(crate) mod shape_access_log;

pub(crate) mod shape_additional_attribute;

pub(crate) mod shape_availability_zones;

pub(crate) mod shape_connection_draining;

pub(crate) mod shape_connection_settings;

pub(crate) mod shape_cross_zone_load_balancing;

pub(crate) mod shape_instance_states;

pub(crate) mod shape_instances;

pub(crate) mod shape_limits;

pub(crate) mod shape_load_balancer_descriptions;

pub(crate) mod shape_policy_descriptions;

pub(crate) mod shape_policy_type_descriptions;

pub(crate) mod shape_security_groups;

pub(crate) mod shape_subnets;

pub(crate) mod shape_tag_descriptions;

pub(crate) mod shape_additional_attributes;

pub(crate) mod shape_instance_state;

pub(crate) mod shape_limit;

pub(crate) mod shape_load_balancer_description;

pub(crate) mod shape_policy_description;

pub(crate) mod shape_policy_type_description;

pub(crate) mod shape_tag_description;

pub(crate) mod shape_backend_server_descriptions;

pub(crate) mod shape_listener_descriptions;

pub(crate) mod shape_policies;

pub(crate) mod shape_policy_attribute_descriptions;

pub(crate) mod shape_policy_attribute_type_descriptions;

pub(crate) mod shape_source_security_group;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_app_cookie_stickiness_policies;

pub(crate) mod shape_backend_server_description;

pub(crate) mod shape_lb_cookie_stickiness_policies;

pub(crate) mod shape_listener_description;

pub(crate) mod shape_policy_attribute_description;

pub(crate) mod shape_policy_attribute_type_description;

pub(crate) mod shape_policy_names;

pub(crate) mod shape_app_cookie_stickiness_policy;

pub(crate) mod shape_lb_cookie_stickiness_policy;

