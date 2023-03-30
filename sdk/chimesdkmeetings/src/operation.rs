// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BatchCreateAttendee`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`batch_create_attendee`](crate::client::fluent_builders::BatchCreateAttendee).
            ///
            /// `ParseStrictResponse` impl for `BatchCreateAttendee`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchCreateAttendee {
    _private: ()
}
impl BatchCreateAttendee {
    /// Creates a new builder-style object to manufacture [`BatchCreateAttendeeInput`](crate::input::BatchCreateAttendeeInput).
    pub fn builder() -> crate::input::batch_create_attendee_input::Builder {
        crate::input::batch_create_attendee_input::Builder::default()
    }
    /// Creates a new `BatchCreateAttendee` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchCreateAttendee {
                type Output = std::result::Result<crate::output::BatchCreateAttendeeOutput, crate::error::BatchCreateAttendeeError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_batch_create_attendee::de_batch_create_attendee_http_error(response)
                     } else {
                        crate::protocol_serde::shape_batch_create_attendee::de_batch_create_attendee_http_response(response)
                     }
                }
            }

/// Operation shape for `BatchUpdateAttendeeCapabilitiesExcept`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`batch_update_attendee_capabilities_except`](crate::client::fluent_builders::BatchUpdateAttendeeCapabilitiesExcept).
            ///
            /// `ParseStrictResponse` impl for `BatchUpdateAttendeeCapabilitiesExcept`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchUpdateAttendeeCapabilitiesExcept {
    _private: ()
}
impl BatchUpdateAttendeeCapabilitiesExcept {
    /// Creates a new builder-style object to manufacture [`BatchUpdateAttendeeCapabilitiesExceptInput`](crate::input::BatchUpdateAttendeeCapabilitiesExceptInput).
    pub fn builder() -> crate::input::batch_update_attendee_capabilities_except_input::Builder {
        crate::input::batch_update_attendee_capabilities_except_input::Builder::default()
    }
    /// Creates a new `BatchUpdateAttendeeCapabilitiesExcept` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchUpdateAttendeeCapabilitiesExcept {
                type Output = std::result::Result<crate::output::BatchUpdateAttendeeCapabilitiesExceptOutput, crate::error::BatchUpdateAttendeeCapabilitiesExceptError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_batch_update_attendee_capabilities_except::de_batch_update_attendee_capabilities_except_http_error(response)
                     } else {
                        crate::protocol_serde::shape_batch_update_attendee_capabilities_except::de_batch_update_attendee_capabilities_except_http_response(response)
                     }
                }
            }

/// Operation shape for `CreateAttendee`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_attendee`](crate::client::fluent_builders::CreateAttendee).
            ///
            /// `ParseStrictResponse` impl for `CreateAttendee`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateAttendee {
    _private: ()
}
impl CreateAttendee {
    /// Creates a new builder-style object to manufacture [`CreateAttendeeInput`](crate::input::CreateAttendeeInput).
    pub fn builder() -> crate::input::create_attendee_input::Builder {
        crate::input::create_attendee_input::Builder::default()
    }
    /// Creates a new `CreateAttendee` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateAttendee {
                type Output = std::result::Result<crate::output::CreateAttendeeOutput, crate::error::CreateAttendeeError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_create_attendee::de_create_attendee_http_error(response)
                     } else {
                        crate::protocol_serde::shape_create_attendee::de_create_attendee_http_response(response)
                     }
                }
            }

/// Operation shape for `CreateMeeting`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_meeting`](crate::client::fluent_builders::CreateMeeting).
            ///
            /// `ParseStrictResponse` impl for `CreateMeeting`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateMeeting {
    _private: ()
}
impl CreateMeeting {
    /// Creates a new builder-style object to manufacture [`CreateMeetingInput`](crate::input::CreateMeetingInput).
    pub fn builder() -> crate::input::create_meeting_input::Builder {
        crate::input::create_meeting_input::Builder::default()
    }
    /// Creates a new `CreateMeeting` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateMeeting {
                type Output = std::result::Result<crate::output::CreateMeetingOutput, crate::error::CreateMeetingError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_create_meeting::de_create_meeting_http_error(response)
                     } else {
                        crate::protocol_serde::shape_create_meeting::de_create_meeting_http_response(response)
                     }
                }
            }

/// Operation shape for `CreateMeetingWithAttendees`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_meeting_with_attendees`](crate::client::fluent_builders::CreateMeetingWithAttendees).
            ///
            /// `ParseStrictResponse` impl for `CreateMeetingWithAttendees`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateMeetingWithAttendees {
    _private: ()
}
impl CreateMeetingWithAttendees {
    /// Creates a new builder-style object to manufacture [`CreateMeetingWithAttendeesInput`](crate::input::CreateMeetingWithAttendeesInput).
    pub fn builder() -> crate::input::create_meeting_with_attendees_input::Builder {
        crate::input::create_meeting_with_attendees_input::Builder::default()
    }
    /// Creates a new `CreateMeetingWithAttendees` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateMeetingWithAttendees {
                type Output = std::result::Result<crate::output::CreateMeetingWithAttendeesOutput, crate::error::CreateMeetingWithAttendeesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_create_meeting_with_attendees::de_create_meeting_with_attendees_http_error(response)
                     } else {
                        crate::protocol_serde::shape_create_meeting_with_attendees::de_create_meeting_with_attendees_http_response(response)
                     }
                }
            }

/// Operation shape for `DeleteAttendee`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_attendee`](crate::client::fluent_builders::DeleteAttendee).
            ///
            /// `ParseStrictResponse` impl for `DeleteAttendee`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteAttendee {
    _private: ()
}
impl DeleteAttendee {
    /// Creates a new builder-style object to manufacture [`DeleteAttendeeInput`](crate::input::DeleteAttendeeInput).
    pub fn builder() -> crate::input::delete_attendee_input::Builder {
        crate::input::delete_attendee_input::Builder::default()
    }
    /// Creates a new `DeleteAttendee` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAttendee {
                type Output = std::result::Result<crate::output::DeleteAttendeeOutput, crate::error::DeleteAttendeeError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 204 {
                        crate::protocol_serde::shape_delete_attendee::de_delete_attendee_http_error(response)
                     } else {
                        crate::protocol_serde::shape_delete_attendee::de_delete_attendee_http_response(response)
                     }
                }
            }

/// Operation shape for `DeleteMeeting`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_meeting`](crate::client::fluent_builders::DeleteMeeting).
            ///
            /// `ParseStrictResponse` impl for `DeleteMeeting`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteMeeting {
    _private: ()
}
impl DeleteMeeting {
    /// Creates a new builder-style object to manufacture [`DeleteMeetingInput`](crate::input::DeleteMeetingInput).
    pub fn builder() -> crate::input::delete_meeting_input::Builder {
        crate::input::delete_meeting_input::Builder::default()
    }
    /// Creates a new `DeleteMeeting` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteMeeting {
                type Output = std::result::Result<crate::output::DeleteMeetingOutput, crate::error::DeleteMeetingError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 204 {
                        crate::protocol_serde::shape_delete_meeting::de_delete_meeting_http_error(response)
                     } else {
                        crate::protocol_serde::shape_delete_meeting::de_delete_meeting_http_response(response)
                     }
                }
            }

/// Operation shape for `GetAttendee`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_attendee`](crate::client::fluent_builders::GetAttendee).
            ///
            /// `ParseStrictResponse` impl for `GetAttendee`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAttendee {
    _private: ()
}
impl GetAttendee {
    /// Creates a new builder-style object to manufacture [`GetAttendeeInput`](crate::input::GetAttendeeInput).
    pub fn builder() -> crate::input::get_attendee_input::Builder {
        crate::input::get_attendee_input::Builder::default()
    }
    /// Creates a new `GetAttendee` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAttendee {
                type Output = std::result::Result<crate::output::GetAttendeeOutput, crate::error::GetAttendeeError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_attendee::de_get_attendee_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_attendee::de_get_attendee_http_response(response)
                     }
                }
            }

/// Operation shape for `GetMeeting`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_meeting`](crate::client::fluent_builders::GetMeeting).
            ///
            /// `ParseStrictResponse` impl for `GetMeeting`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetMeeting {
    _private: ()
}
impl GetMeeting {
    /// Creates a new builder-style object to manufacture [`GetMeetingInput`](crate::input::GetMeetingInput).
    pub fn builder() -> crate::input::get_meeting_input::Builder {
        crate::input::get_meeting_input::Builder::default()
    }
    /// Creates a new `GetMeeting` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetMeeting {
                type Output = std::result::Result<crate::output::GetMeetingOutput, crate::error::GetMeetingError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_get_meeting::de_get_meeting_http_error(response)
                     } else {
                        crate::protocol_serde::shape_get_meeting::de_get_meeting_http_response(response)
                     }
                }
            }

/// Operation shape for `ListAttendees`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_attendees`](crate::client::fluent_builders::ListAttendees).
            ///
            /// `ParseStrictResponse` impl for `ListAttendees`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListAttendees {
    _private: ()
}
impl ListAttendees {
    /// Creates a new builder-style object to manufacture [`ListAttendeesInput`](crate::input::ListAttendeesInput).
    pub fn builder() -> crate::input::list_attendees_input::Builder {
        crate::input::list_attendees_input::Builder::default()
    }
    /// Creates a new `ListAttendees` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAttendees {
                type Output = std::result::Result<crate::output::ListAttendeesOutput, crate::error::ListAttendeesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_attendees::de_list_attendees_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_attendees::de_list_attendees_http_response(response)
                     }
                }
            }

/// Operation shape for `ListTagsForResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_tags_for_resource`](crate::client::fluent_builders::ListTagsForResource).
            ///
            /// `ParseStrictResponse` impl for `ListTagsForResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: ()
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
                type Output = std::result::Result<crate::output::ListTagsForResourceOutput, crate::error::ListTagsForResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_list_tags_for_resource::de_list_tags_for_resource_http_error(response)
                     } else {
                        crate::protocol_serde::shape_list_tags_for_resource::de_list_tags_for_resource_http_response(response)
                     }
                }
            }

/// Operation shape for `StartMeetingTranscription`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_meeting_transcription`](crate::client::fluent_builders::StartMeetingTranscription).
            ///
            /// `ParseStrictResponse` impl for `StartMeetingTranscription`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartMeetingTranscription {
    _private: ()
}
impl StartMeetingTranscription {
    /// Creates a new builder-style object to manufacture [`StartMeetingTranscriptionInput`](crate::input::StartMeetingTranscriptionInput).
    pub fn builder() -> crate::input::start_meeting_transcription_input::Builder {
        crate::input::start_meeting_transcription_input::Builder::default()
    }
    /// Creates a new `StartMeetingTranscription` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartMeetingTranscription {
                type Output = std::result::Result<crate::output::StartMeetingTranscriptionOutput, crate::error::StartMeetingTranscriptionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_start_meeting_transcription::de_start_meeting_transcription_http_error(response)
                     } else {
                        crate::protocol_serde::shape_start_meeting_transcription::de_start_meeting_transcription_http_response(response)
                     }
                }
            }

/// Operation shape for `StopMeetingTranscription`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`stop_meeting_transcription`](crate::client::fluent_builders::StopMeetingTranscription).
            ///
            /// `ParseStrictResponse` impl for `StopMeetingTranscription`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StopMeetingTranscription {
    _private: ()
}
impl StopMeetingTranscription {
    /// Creates a new builder-style object to manufacture [`StopMeetingTranscriptionInput`](crate::input::StopMeetingTranscriptionInput).
    pub fn builder() -> crate::input::stop_meeting_transcription_input::Builder {
        crate::input::stop_meeting_transcription_input::Builder::default()
    }
    /// Creates a new `StopMeetingTranscription` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopMeetingTranscription {
                type Output = std::result::Result<crate::output::StopMeetingTranscriptionOutput, crate::error::StopMeetingTranscriptionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_stop_meeting_transcription::de_stop_meeting_transcription_http_error(response)
                     } else {
                        crate::protocol_serde::shape_stop_meeting_transcription::de_stop_meeting_transcription_http_response(response)
                     }
                }
            }

/// Operation shape for `TagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`tag_resource`](crate::client::fluent_builders::TagResource).
            ///
            /// `ParseStrictResponse` impl for `TagResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagResource {
    _private: ()
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
                type Output = std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 204 {
                        crate::protocol_serde::shape_tag_resource::de_tag_resource_http_error(response)
                     } else {
                        crate::protocol_serde::shape_tag_resource::de_tag_resource_http_response(response)
                     }
                }
            }

/// Operation shape for `UntagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`untag_resource`](crate::client::fluent_builders::UntagResource).
            ///
            /// `ParseStrictResponse` impl for `UntagResource`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagResource {
    _private: ()
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
                type Output = std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 204 {
                        crate::protocol_serde::shape_untag_resource::de_untag_resource_http_error(response)
                     } else {
                        crate::protocol_serde::shape_untag_resource::de_untag_resource_http_response(response)
                     }
                }
            }

/// Operation shape for `UpdateAttendeeCapabilities`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_attendee_capabilities`](crate::client::fluent_builders::UpdateAttendeeCapabilities).
            ///
            /// `ParseStrictResponse` impl for `UpdateAttendeeCapabilities`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateAttendeeCapabilities {
    _private: ()
}
impl UpdateAttendeeCapabilities {
    /// Creates a new builder-style object to manufacture [`UpdateAttendeeCapabilitiesInput`](crate::input::UpdateAttendeeCapabilitiesInput).
    pub fn builder() -> crate::input::update_attendee_capabilities_input::Builder {
        crate::input::update_attendee_capabilities_input::Builder::default()
    }
    /// Creates a new `UpdateAttendeeCapabilities` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateAttendeeCapabilities {
                type Output = std::result::Result<crate::output::UpdateAttendeeCapabilitiesOutput, crate::error::UpdateAttendeeCapabilitiesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::protocol_serde::shape_update_attendee_capabilities::de_update_attendee_capabilities_http_error(response)
                     } else {
                        crate::protocol_serde::shape_update_attendee_capabilities::de_update_attendee_capabilities_http_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

