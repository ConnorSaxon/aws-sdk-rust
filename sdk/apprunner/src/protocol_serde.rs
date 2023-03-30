// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_associate_custom_domain;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_create_auto_scaling_configuration;

pub(crate) mod shape_create_connection;

pub(crate) mod shape_create_observability_configuration;

pub(crate) mod shape_create_service;

pub(crate) mod shape_create_vpc_connector;

pub(crate) mod shape_create_vpc_ingress_connection;

pub(crate) mod shape_delete_auto_scaling_configuration;

pub(crate) mod shape_delete_connection;

pub(crate) mod shape_delete_observability_configuration;

pub(crate) mod shape_delete_service;

pub(crate) mod shape_delete_vpc_connector;

pub(crate) mod shape_delete_vpc_ingress_connection;

pub(crate) mod shape_describe_auto_scaling_configuration;

pub(crate) mod shape_describe_custom_domains;

pub(crate) mod shape_describe_observability_configuration;

pub(crate) mod shape_describe_service;

pub(crate) mod shape_describe_vpc_connector;

pub(crate) mod shape_describe_vpc_ingress_connection;

pub(crate) mod shape_disassociate_custom_domain;

pub(crate) mod shape_list_auto_scaling_configurations;

pub(crate) mod shape_list_connections;

pub(crate) mod shape_list_observability_configurations;

pub(crate) mod shape_list_operations;

pub(crate) mod shape_list_services;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_vpc_connectors;

pub(crate) mod shape_list_vpc_ingress_connections;

pub(crate) mod shape_pause_service;

pub(crate) mod shape_resume_service;

pub(crate) mod shape_start_deployment;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_service;

pub(crate) mod shape_update_vpc_ingress_connection;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_associate_custom_domain_input;

pub(crate) mod shape_create_auto_scaling_configuration_input;

pub(crate) mod shape_create_connection_input;

pub(crate) mod shape_create_observability_configuration_input;

pub(crate) mod shape_create_service_input;

pub(crate) mod shape_create_vpc_connector_input;

pub(crate) mod shape_create_vpc_ingress_connection_input;

pub(crate) mod shape_delete_auto_scaling_configuration_input;

pub(crate) mod shape_delete_connection_input;

pub(crate) mod shape_delete_observability_configuration_input;

pub(crate) mod shape_delete_service_input;

pub(crate) mod shape_delete_vpc_connector_input;

pub(crate) mod shape_delete_vpc_ingress_connection_input;

pub(crate) mod shape_describe_auto_scaling_configuration_input;

pub(crate) mod shape_describe_custom_domains_input;

pub(crate) mod shape_describe_observability_configuration_input;

pub(crate) mod shape_describe_service_input;

pub(crate) mod shape_describe_vpc_connector_input;

pub(crate) mod shape_describe_vpc_ingress_connection_input;

pub(crate) mod shape_disassociate_custom_domain_input;

pub(crate) mod shape_internal_service_error_exception;

pub(crate) mod shape_invalid_request_exception;

pub(crate) mod shape_invalid_state_exception;

pub(crate) mod shape_list_auto_scaling_configurations_input;

pub(crate) mod shape_list_connections_input;

pub(crate) mod shape_list_observability_configurations_input;

pub(crate) mod shape_list_operations_input;

pub(crate) mod shape_list_services_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_list_vpc_connectors_input;

pub(crate) mod shape_list_vpc_ingress_connections_input;

pub(crate) mod shape_pause_service_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_resume_service_input;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_start_deployment_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_service_input;

pub(crate) mod shape_update_vpc_ingress_connection_input;

pub(crate) mod shape_auto_scaling_configuration;

pub(crate) mod shape_auto_scaling_configuration_summary_list;

pub(crate) mod shape_connection;

pub(crate) mod shape_connection_summary_list;

pub(crate) mod shape_custom_domain;

pub(crate) mod shape_custom_domain_list;

pub(crate) mod shape_encryption_configuration;

pub(crate) mod shape_health_check_configuration;

pub(crate) mod shape_ingress_vpc_configuration;

pub(crate) mod shape_instance_configuration;

pub(crate) mod shape_list_vpc_ingress_connections_filter;

pub(crate) mod shape_network_configuration;

pub(crate) mod shape_observability_configuration;

pub(crate) mod shape_observability_configuration_summary_list;

pub(crate) mod shape_operation_summary_list;

pub(crate) mod shape_service;

pub(crate) mod shape_service_observability_configuration;

pub(crate) mod shape_service_summary_list;

pub(crate) mod shape_source_configuration;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_trace_configuration;

pub(crate) mod shape_vpc_connector;

pub(crate) mod shape_vpc_connectors;

pub(crate) mod shape_vpc_dns_target_list;

pub(crate) mod shape_vpc_ingress_connection;

pub(crate) mod shape_vpc_ingress_connection_summary_list;

pub(crate) mod shape_authentication_configuration;

pub(crate) mod shape_auto_scaling_configuration_summary;

pub(crate) mod shape_certificate_validation_record_list;

pub(crate) mod shape_code_repository;

pub(crate) mod shape_connection_summary;

pub(crate) mod shape_egress_configuration;

pub(crate) mod shape_image_repository;

pub(crate) mod shape_ingress_configuration;

pub(crate) mod shape_observability_configuration_summary;

pub(crate) mod shape_operation_summary;

pub(crate) mod shape_service_summary;

pub(crate) mod shape_string_list;

pub(crate) mod shape_vpc_dns_target;

pub(crate) mod shape_vpc_ingress_connection_summary;

pub(crate) mod shape_certificate_validation_record;

pub(crate) mod shape_code_configuration;

pub(crate) mod shape_image_configuration;

pub(crate) mod shape_source_code_version;

pub(crate) mod shape_code_configuration_values;

pub(crate) mod shape_runtime_environment_secrets;

pub(crate) mod shape_runtime_environment_variables;

