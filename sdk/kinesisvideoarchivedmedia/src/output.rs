// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ListFragmentsOutput {
    /// <p>A list of archived <code>Fragment</code> objects from the stream that meet the selector criteria. Results are in no specific order, even across pages.</p>
    pub fragments: std::option::Option<std::vec::Vec<crate::model::Fragment>>,
    /// <p>If the returned list is truncated, the operation returns this token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl ListFragmentsOutput {
    /// <p>A list of archived <code>Fragment</code> objects from the stream that meet the selector criteria. Results are in no specific order, even across pages.</p>
    pub fn fragments(&self) -> std::option::Option<&[crate::model::Fragment]> {
        self.fragments.as_deref()
    }
    /// <p>If the returned list is truncated, the operation returns this token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for ListFragmentsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListFragmentsOutput");
        formatter.field("fragments", &self.fragments);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListFragmentsOutput`](crate::output::ListFragmentsOutput)
pub mod list_fragments_output {
    /// A builder for [`ListFragmentsOutput`](crate::output::ListFragmentsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) fragments: std::option::Option<std::vec::Vec<crate::model::Fragment>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `fragments`.
        ///
        /// To override the contents of this collection use [`set_fragments`](Self::set_fragments).
        ///
        /// <p>A list of archived <code>Fragment</code> objects from the stream that meet the selector criteria. Results are in no specific order, even across pages.</p>
        pub fn fragments(mut self, input: crate::model::Fragment) -> Self {
            let mut v = self.fragments.unwrap_or_default();
            v.push(input);
            self.fragments = Some(v);
            self
        }
        /// <p>A list of archived <code>Fragment</code> objects from the stream that meet the selector criteria. Results are in no specific order, even across pages.</p>
        pub fn set_fragments(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Fragment>>,
        ) -> Self {
            self.fragments = input;
            self
        }
        /// <p>If the returned list is truncated, the operation returns this token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>If the returned list is truncated, the operation returns this token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListFragmentsOutput`](crate::output::ListFragmentsOutput)
        pub fn build(self) -> crate::output::ListFragmentsOutput {
            crate::output::ListFragmentsOutput {
                fragments: self.fragments,
                next_token: self.next_token,
            }
        }
    }
}
impl ListFragmentsOutput {
    /// Creates a new builder-style object to manufacture [`ListFragmentsOutput`](crate::output::ListFragmentsOutput)
    pub fn builder() -> crate::output::list_fragments_output::Builder {
        crate::output::list_fragments_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
pub struct GetMediaForFragmentListOutput {
    /// <p>The content type of the requested media.</p>
    pub content_type: std::option::Option<std::string::String>,
    /// <p>The payload that Kinesis Video Streams returns is a sequence of chunks from the specified stream. For information about the chunks, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_dataplane_PutMedia.html">PutMedia</a>. The chunks that Kinesis Video Streams returns in the <code>GetMediaForFragmentList</code> call also include the following additional Matroska (MKV) tags: </p>
    /// <ul>
    /// <li> <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - Fragment number returned in the chunk.</p> </li>
    /// <li> <p>AWS_KINESISVIDEO_SERVER_SIDE_TIMESTAMP - Server-side timestamp of the fragment.</p> </li>
    /// <li> <p>AWS_KINESISVIDEO_PRODUCER_SIDE_TIMESTAMP - Producer-side timestamp of the fragment.</p> </li>
    /// </ul>
    /// <p>The following tags will be included if an exception occurs:</p>
    /// <ul>
    /// <li> <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - The number of the fragment that threw the exception</p> </li>
    /// <li> <p>AWS_KINESISVIDEO_EXCEPTION_ERROR_CODE - The integer code of the exception</p> </li>
    /// <li> <p>AWS_KINESISVIDEO_EXCEPTION_MESSAGE - A text description of the exception</p> </li>
    /// </ul>
    pub payload: aws_smithy_http::byte_stream::ByteStream,
}
impl GetMediaForFragmentListOutput {
    /// <p>The content type of the requested media.</p>
    pub fn content_type(&self) -> std::option::Option<&str> {
        self.content_type.as_deref()
    }
    /// <p>The payload that Kinesis Video Streams returns is a sequence of chunks from the specified stream. For information about the chunks, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_dataplane_PutMedia.html">PutMedia</a>. The chunks that Kinesis Video Streams returns in the <code>GetMediaForFragmentList</code> call also include the following additional Matroska (MKV) tags: </p>
    /// <ul>
    /// <li> <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - Fragment number returned in the chunk.</p> </li>
    /// <li> <p>AWS_KINESISVIDEO_SERVER_SIDE_TIMESTAMP - Server-side timestamp of the fragment.</p> </li>
    /// <li> <p>AWS_KINESISVIDEO_PRODUCER_SIDE_TIMESTAMP - Producer-side timestamp of the fragment.</p> </li>
    /// </ul>
    /// <p>The following tags will be included if an exception occurs:</p>
    /// <ul>
    /// <li> <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - The number of the fragment that threw the exception</p> </li>
    /// <li> <p>AWS_KINESISVIDEO_EXCEPTION_ERROR_CODE - The integer code of the exception</p> </li>
    /// <li> <p>AWS_KINESISVIDEO_EXCEPTION_MESSAGE - A text description of the exception</p> </li>
    /// </ul>
    pub fn payload(&self) -> &aws_smithy_http::byte_stream::ByteStream {
        &self.payload
    }
}
impl std::fmt::Debug for GetMediaForFragmentListOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetMediaForFragmentListOutput");
        formatter.field("content_type", &self.content_type);
        formatter.field("payload", &self.payload);
        formatter.finish()
    }
}
/// See [`GetMediaForFragmentListOutput`](crate::output::GetMediaForFragmentListOutput)
pub mod get_media_for_fragment_list_output {
    /// A builder for [`GetMediaForFragmentListOutput`](crate::output::GetMediaForFragmentListOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) content_type: std::option::Option<std::string::String>,
        pub(crate) payload: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
    }
    impl Builder {
        /// <p>The content type of the requested media.</p>
        pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.content_type = Some(input.into());
            self
        }
        /// <p>The content type of the requested media.</p>
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.content_type = input;
            self
        }
        /// <p>The payload that Kinesis Video Streams returns is a sequence of chunks from the specified stream. For information about the chunks, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_dataplane_PutMedia.html">PutMedia</a>. The chunks that Kinesis Video Streams returns in the <code>GetMediaForFragmentList</code> call also include the following additional Matroska (MKV) tags: </p>
        /// <ul>
        /// <li> <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - Fragment number returned in the chunk.</p> </li>
        /// <li> <p>AWS_KINESISVIDEO_SERVER_SIDE_TIMESTAMP - Server-side timestamp of the fragment.</p> </li>
        /// <li> <p>AWS_KINESISVIDEO_PRODUCER_SIDE_TIMESTAMP - Producer-side timestamp of the fragment.</p> </li>
        /// </ul>
        /// <p>The following tags will be included if an exception occurs:</p>
        /// <ul>
        /// <li> <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - The number of the fragment that threw the exception</p> </li>
        /// <li> <p>AWS_KINESISVIDEO_EXCEPTION_ERROR_CODE - The integer code of the exception</p> </li>
        /// <li> <p>AWS_KINESISVIDEO_EXCEPTION_MESSAGE - A text description of the exception</p> </li>
        /// </ul>
        pub fn payload(mut self, input: aws_smithy_http::byte_stream::ByteStream) -> Self {
            self.payload = Some(input);
            self
        }
        /// <p>The payload that Kinesis Video Streams returns is a sequence of chunks from the specified stream. For information about the chunks, see <a href="http://docs.aws.amazon.com/kinesisvideostreams/latest/dg/API_dataplane_PutMedia.html">PutMedia</a>. The chunks that Kinesis Video Streams returns in the <code>GetMediaForFragmentList</code> call also include the following additional Matroska (MKV) tags: </p>
        /// <ul>
        /// <li> <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - Fragment number returned in the chunk.</p> </li>
        /// <li> <p>AWS_KINESISVIDEO_SERVER_SIDE_TIMESTAMP - Server-side timestamp of the fragment.</p> </li>
        /// <li> <p>AWS_KINESISVIDEO_PRODUCER_SIDE_TIMESTAMP - Producer-side timestamp of the fragment.</p> </li>
        /// </ul>
        /// <p>The following tags will be included if an exception occurs:</p>
        /// <ul>
        /// <li> <p>AWS_KINESISVIDEO_FRAGMENT_NUMBER - The number of the fragment that threw the exception</p> </li>
        /// <li> <p>AWS_KINESISVIDEO_EXCEPTION_ERROR_CODE - The integer code of the exception</p> </li>
        /// <li> <p>AWS_KINESISVIDEO_EXCEPTION_MESSAGE - A text description of the exception</p> </li>
        /// </ul>
        pub fn set_payload(
            mut self,
            input: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
        ) -> Self {
            self.payload = input;
            self
        }
        /// Consumes the builder and constructs a [`GetMediaForFragmentListOutput`](crate::output::GetMediaForFragmentListOutput)
        pub fn build(self) -> crate::output::GetMediaForFragmentListOutput {
            crate::output::GetMediaForFragmentListOutput {
                content_type: self.content_type,
                payload: self.payload.unwrap_or_default(),
            }
        }
    }
}
impl GetMediaForFragmentListOutput {
    /// Creates a new builder-style object to manufacture [`GetMediaForFragmentListOutput`](crate::output::GetMediaForFragmentListOutput)
    pub fn builder() -> crate::output::get_media_for_fragment_list_output::Builder {
        crate::output::get_media_for_fragment_list_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetHlsStreamingSessionUrlOutput {
    /// <p>The URL (containing the session token) that a media player can use to retrieve the HLS master playlist.</p>
    pub hls_streaming_session_url: std::option::Option<std::string::String>,
}
impl GetHlsStreamingSessionUrlOutput {
    /// <p>The URL (containing the session token) that a media player can use to retrieve the HLS master playlist.</p>
    pub fn hls_streaming_session_url(&self) -> std::option::Option<&str> {
        self.hls_streaming_session_url.as_deref()
    }
}
impl std::fmt::Debug for GetHlsStreamingSessionUrlOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetHlsStreamingSessionUrlOutput");
        formatter.field("hls_streaming_session_url", &self.hls_streaming_session_url);
        formatter.finish()
    }
}
/// See [`GetHlsStreamingSessionUrlOutput`](crate::output::GetHlsStreamingSessionUrlOutput)
pub mod get_hls_streaming_session_url_output {
    /// A builder for [`GetHlsStreamingSessionUrlOutput`](crate::output::GetHlsStreamingSessionUrlOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) hls_streaming_session_url: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The URL (containing the session token) that a media player can use to retrieve the HLS master playlist.</p>
        pub fn hls_streaming_session_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.hls_streaming_session_url = Some(input.into());
            self
        }
        /// <p>The URL (containing the session token) that a media player can use to retrieve the HLS master playlist.</p>
        pub fn set_hls_streaming_session_url(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.hls_streaming_session_url = input;
            self
        }
        /// Consumes the builder and constructs a [`GetHlsStreamingSessionUrlOutput`](crate::output::GetHlsStreamingSessionUrlOutput)
        pub fn build(self) -> crate::output::GetHlsStreamingSessionUrlOutput {
            crate::output::GetHlsStreamingSessionUrlOutput {
                hls_streaming_session_url: self.hls_streaming_session_url,
            }
        }
    }
}
impl GetHlsStreamingSessionUrlOutput {
    /// Creates a new builder-style object to manufacture [`GetHlsStreamingSessionUrlOutput`](crate::output::GetHlsStreamingSessionUrlOutput)
    pub fn builder() -> crate::output::get_hls_streaming_session_url_output::Builder {
        crate::output::get_hls_streaming_session_url_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetDashStreamingSessionUrlOutput {
    /// <p>The URL (containing the session token) that a media player can use to retrieve the MPEG-DASH manifest.</p>
    pub dash_streaming_session_url: std::option::Option<std::string::String>,
}
impl GetDashStreamingSessionUrlOutput {
    /// <p>The URL (containing the session token) that a media player can use to retrieve the MPEG-DASH manifest.</p>
    pub fn dash_streaming_session_url(&self) -> std::option::Option<&str> {
        self.dash_streaming_session_url.as_deref()
    }
}
impl std::fmt::Debug for GetDashStreamingSessionUrlOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetDashStreamingSessionUrlOutput");
        formatter.field(
            "dash_streaming_session_url",
            &self.dash_streaming_session_url,
        );
        formatter.finish()
    }
}
/// See [`GetDashStreamingSessionUrlOutput`](crate::output::GetDashStreamingSessionUrlOutput)
pub mod get_dash_streaming_session_url_output {
    /// A builder for [`GetDashStreamingSessionUrlOutput`](crate::output::GetDashStreamingSessionUrlOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) dash_streaming_session_url: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The URL (containing the session token) that a media player can use to retrieve the MPEG-DASH manifest.</p>
        pub fn dash_streaming_session_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.dash_streaming_session_url = Some(input.into());
            self
        }
        /// <p>The URL (containing the session token) that a media player can use to retrieve the MPEG-DASH manifest.</p>
        pub fn set_dash_streaming_session_url(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.dash_streaming_session_url = input;
            self
        }
        /// Consumes the builder and constructs a [`GetDashStreamingSessionUrlOutput`](crate::output::GetDashStreamingSessionUrlOutput)
        pub fn build(self) -> crate::output::GetDashStreamingSessionUrlOutput {
            crate::output::GetDashStreamingSessionUrlOutput {
                dash_streaming_session_url: self.dash_streaming_session_url,
            }
        }
    }
}
impl GetDashStreamingSessionUrlOutput {
    /// Creates a new builder-style object to manufacture [`GetDashStreamingSessionUrlOutput`](crate::output::GetDashStreamingSessionUrlOutput)
    pub fn builder() -> crate::output::get_dash_streaming_session_url_output::Builder {
        crate::output::get_dash_streaming_session_url_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
pub struct GetClipOutput {
    /// <p>The content type of the media in the requested clip.</p>
    pub content_type: std::option::Option<std::string::String>,
    /// <p>Traditional MP4 file that contains the media clip from the specified video stream. The output will contain the first 100 MB or the first 200 fragments from the specified start timestamp. For more information, see <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/limits.html">Kinesis Video Streams Limits</a>. </p>
    pub payload: aws_smithy_http::byte_stream::ByteStream,
}
impl GetClipOutput {
    /// <p>The content type of the media in the requested clip.</p>
    pub fn content_type(&self) -> std::option::Option<&str> {
        self.content_type.as_deref()
    }
    /// <p>Traditional MP4 file that contains the media clip from the specified video stream. The output will contain the first 100 MB or the first 200 fragments from the specified start timestamp. For more information, see <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/limits.html">Kinesis Video Streams Limits</a>. </p>
    pub fn payload(&self) -> &aws_smithy_http::byte_stream::ByteStream {
        &self.payload
    }
}
impl std::fmt::Debug for GetClipOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetClipOutput");
        formatter.field("content_type", &self.content_type);
        formatter.field("payload", &self.payload);
        formatter.finish()
    }
}
/// See [`GetClipOutput`](crate::output::GetClipOutput)
pub mod get_clip_output {
    /// A builder for [`GetClipOutput`](crate::output::GetClipOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) content_type: std::option::Option<std::string::String>,
        pub(crate) payload: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
    }
    impl Builder {
        /// <p>The content type of the media in the requested clip.</p>
        pub fn content_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.content_type = Some(input.into());
            self
        }
        /// <p>The content type of the media in the requested clip.</p>
        pub fn set_content_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.content_type = input;
            self
        }
        /// <p>Traditional MP4 file that contains the media clip from the specified video stream. The output will contain the first 100 MB or the first 200 fragments from the specified start timestamp. For more information, see <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/limits.html">Kinesis Video Streams Limits</a>. </p>
        pub fn payload(mut self, input: aws_smithy_http::byte_stream::ByteStream) -> Self {
            self.payload = Some(input);
            self
        }
        /// <p>Traditional MP4 file that contains the media clip from the specified video stream. The output will contain the first 100 MB or the first 200 fragments from the specified start timestamp. For more information, see <a href="https://docs.aws.amazon.com/kinesisvideostreams/latest/dg/limits.html">Kinesis Video Streams Limits</a>. </p>
        pub fn set_payload(
            mut self,
            input: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
        ) -> Self {
            self.payload = input;
            self
        }
        /// Consumes the builder and constructs a [`GetClipOutput`](crate::output::GetClipOutput)
        pub fn build(self) -> crate::output::GetClipOutput {
            crate::output::GetClipOutput {
                content_type: self.content_type,
                payload: self.payload.unwrap_or_default(),
            }
        }
    }
}
impl GetClipOutput {
    /// Creates a new builder-style object to manufacture [`GetClipOutput`](crate::output::GetClipOutput)
    pub fn builder() -> crate::output::get_clip_output::Builder {
        crate::output::get_clip_output::Builder::default()
    }
}
