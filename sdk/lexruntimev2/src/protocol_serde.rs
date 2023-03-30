// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_error_metadata(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::error::metadata::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
                    crate::json_errors::parse_error_metadata(response.body(), response.headers())
                }

pub(crate) mod shape_delete_session;

pub(crate) mod shape_get_session;

pub(crate) mod shape_put_session;

pub(crate) mod shape_recognize_text;

pub(crate) mod shape_recognize_utterance;

pub(crate) mod shape_recognize_utterance_input;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_bad_gateway_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_dependency_failed_exception;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_put_session_input;

pub(crate) mod shape_put_session_output;

pub(crate) mod shape_recognize_text_input;

pub(crate) mod shape_recognize_utterance_output;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_interpretations;

pub(crate) mod shape_message;

pub(crate) mod shape_messages;

pub(crate) mod shape_session_state;

pub(crate) mod shape_string_map;

pub(crate) mod shape_active_context;

pub(crate) mod shape_active_contexts_list;

pub(crate) mod shape_dialog_action;

pub(crate) mod shape_image_response_card;

pub(crate) mod shape_intent;

pub(crate) mod shape_interpretation;

pub(crate) mod shape_runtime_hints;

pub(crate) mod shape_active_context_time_to_live;

pub(crate) mod shape_button;

pub(crate) mod shape_confidence_score;

pub(crate) mod shape_elicit_sub_slot;

pub(crate) mod shape_runtime_hint_details;

pub(crate) mod shape_sentiment_response;

pub(crate) mod shape_slot;

pub(crate) mod shape_slot_hints_intent_map;

pub(crate) mod shape_slots;

pub(crate) mod shape_active_context_parameters_map;

pub(crate) mod shape_buttons_list;

pub(crate) mod shape_runtime_hint_value;

pub(crate) mod shape_sentiment_score;

pub(crate) mod shape_slot_hints_slot_map;

pub(crate) mod shape_value;

pub(crate) mod shape_values;

pub(crate) mod shape_runtime_hint_values_list;

pub(crate) mod shape_string_list;

