// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_create_api;

pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_create_api_mapping;

pub(crate) mod shape_create_authorizer;

pub(crate) mod shape_create_deployment;

pub(crate) mod shape_create_domain_name;

pub(crate) mod shape_create_integration;

pub(crate) mod shape_create_integration_response;

pub(crate) mod shape_create_model;

pub(crate) mod shape_create_route;

pub(crate) mod shape_create_route_response;

pub(crate) mod shape_create_stage;

pub(crate) mod shape_create_vpc_link;

pub(crate) mod shape_delete_access_log_settings;

pub(crate) mod shape_delete_api;

pub(crate) mod shape_delete_api_mapping;

pub(crate) mod shape_delete_authorizer;

pub(crate) mod shape_delete_cors_configuration;

pub(crate) mod shape_delete_deployment;

pub(crate) mod shape_delete_domain_name;

pub(crate) mod shape_delete_integration;

pub(crate) mod shape_delete_integration_response;

pub(crate) mod shape_delete_model;

pub(crate) mod shape_delete_route;

pub(crate) mod shape_delete_route_request_parameter;

pub(crate) mod shape_delete_route_response;

pub(crate) mod shape_delete_route_settings;

pub(crate) mod shape_delete_stage;

pub(crate) mod shape_delete_vpc_link;

pub(crate) mod shape_export_api;

pub(crate) mod shape_get_api;

pub(crate) mod shape_get_api_mapping;

pub(crate) mod shape_get_api_mappings;

pub(crate) mod shape_get_apis;

pub(crate) mod shape_get_authorizer;

pub(crate) mod shape_get_authorizers;

pub(crate) mod shape_get_deployment;

pub(crate) mod shape_get_deployments;

pub(crate) mod shape_get_domain_name;

pub(crate) mod shape_get_domain_names;

pub(crate) mod shape_get_integration;

pub(crate) mod shape_get_integration_response;

pub(crate) mod shape_get_integration_responses;

pub(crate) mod shape_get_integrations;

pub(crate) mod shape_get_model;

pub(crate) mod shape_get_model_template;

pub(crate) mod shape_get_models;

pub(crate) mod shape_get_route;

pub(crate) mod shape_get_route_response;

pub(crate) mod shape_get_route_responses;

pub(crate) mod shape_get_routes;

pub(crate) mod shape_get_stage;

pub(crate) mod shape_get_stages;

pub(crate) mod shape_get_tags;

pub(crate) mod shape_get_vpc_link;

pub(crate) mod shape_get_vpc_links;

pub(crate) mod shape_import_api;

pub(crate) mod shape_reimport_api;

pub(crate) mod shape_reset_authorizers_cache;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_api;

pub(crate) mod shape_update_api_mapping;

pub(crate) mod shape_update_authorizer;

pub(crate) mod shape_update_deployment;

pub(crate) mod shape_update_domain_name;

pub(crate) mod shape_update_integration;

pub(crate) mod shape_update_integration_response;

pub(crate) mod shape_update_model;

pub(crate) mod shape_update_route;

pub(crate) mod shape_update_route_response;

pub(crate) mod shape_update_stage;

pub(crate) mod shape_update_vpc_link;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_api_input;

pub(crate) mod shape_create_api_mapping_input;

pub(crate) mod shape_create_authorizer_input;

pub(crate) mod shape_create_deployment_input;

pub(crate) mod shape_create_domain_name_input;

pub(crate) mod shape_create_integration_input;

pub(crate) mod shape_create_integration_response_input;

pub(crate) mod shape_create_model_input;

pub(crate) mod shape_create_route_input;

pub(crate) mod shape_create_route_response_input;

pub(crate) mod shape_create_stage_input;

pub(crate) mod shape_create_vpc_link_input;

pub(crate) mod shape_export_api_output;

pub(crate) mod shape_import_api_input;

pub(crate) mod shape_not_found_exception;

pub(crate) mod shape_reimport_api_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_too_many_requests_exception;

pub(crate) mod shape_update_api_input;

pub(crate) mod shape_update_api_mapping_input;

pub(crate) mod shape_update_authorizer_input;

pub(crate) mod shape_update_deployment_input;

pub(crate) mod shape_update_domain_name_input;

pub(crate) mod shape_update_integration_input;

pub(crate) mod shape_update_integration_response_input;

pub(crate) mod shape_update_model_input;

pub(crate) mod shape_update_route_input;

pub(crate) mod shape_update_route_response_input;

pub(crate) mod shape_update_stage_input;

pub(crate) mod shape_update_vpc_link_input;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of__string;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_api;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_api_mapping;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_authorizer;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_deployment;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_domain_name;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_integration;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_integration_response;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_model;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_route;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_route_response;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_stage;

#[allow(non_snake_case)]
pub(crate) mod shape___list_of_vpc_link;

pub(crate) mod shape_access_log_settings;

pub(crate) mod shape_authorization_scopes;

pub(crate) mod shape_cors;

pub(crate) mod shape_domain_name_configuration;

pub(crate) mod shape_domain_name_configurations;

pub(crate) mod shape_identity_source_list;

pub(crate) mod shape_integration_parameters;

pub(crate) mod shape_jwt_configuration;

pub(crate) mod shape_mutual_tls_authentication;

pub(crate) mod shape_mutual_tls_authentication_input;

pub(crate) mod shape_parameter_constraints;

pub(crate) mod shape_response_parameters;

pub(crate) mod shape_route_models;

pub(crate) mod shape_route_parameters;

pub(crate) mod shape_route_settings;

pub(crate) mod shape_route_settings_map;

pub(crate) mod shape_security_group_id_list;

pub(crate) mod shape_stage_variables_map;

pub(crate) mod shape_subnet_id_list;

pub(crate) mod shape_tags;

pub(crate) mod shape_template_map;

pub(crate) mod shape_tls_config;

pub(crate) mod shape_tls_config_input;

pub(crate) mod shape_api;

pub(crate) mod shape_api_mapping;

pub(crate) mod shape_authorizer;

pub(crate) mod shape_cors_header_list;

pub(crate) mod shape_cors_method_list;

pub(crate) mod shape_cors_origin_list;

pub(crate) mod shape_deployment;

pub(crate) mod shape_domain_name;

pub(crate) mod shape_integration;

pub(crate) mod shape_integration_response;

pub(crate) mod shape_model;

pub(crate) mod shape_route;

pub(crate) mod shape_route_response;

pub(crate) mod shape_stage;

pub(crate) mod shape_vpc_link;

