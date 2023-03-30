// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutBucketTagging`](crate::client::fluent_builders::PutBucketTagging) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::client::fluent_builders::PutBucketTagging::bucket) / [`set_bucket(Option<String>)`](crate::client::fluent_builders::PutBucketTagging::set_bucket): <p>The bucket name.</p>
    ///   - [`content_md5(impl Into<String>)`](crate::client::fluent_builders::PutBucketTagging::content_md5) / [`set_content_md5(Option<String>)`](crate::client::fluent_builders::PutBucketTagging::set_content_md5): <p>The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>  <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    ///   - [`checksum_algorithm(ChecksumAlgorithm)`](crate::client::fluent_builders::PutBucketTagging::checksum_algorithm) / [`set_checksum_algorithm(Option<ChecksumAlgorithm>)`](crate::client::fluent_builders::PutBucketTagging::set_checksum_algorithm): <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    ///   - [`tagging(Tagging)`](crate::client::fluent_builders::PutBucketTagging::tagging) / [`set_tagging(Option<Tagging>)`](crate::client::fluent_builders::PutBucketTagging::set_tagging): <p>Container for the <code>TagSet</code> and <code>Tag</code> elements.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::client::fluent_builders::PutBucketTagging::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::client::fluent_builders::PutBucketTagging::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
                            /// - On success, responds with [`PutBucketTaggingOutput`](crate::output::PutBucketTaggingOutput)
                            /// - On failure, responds with [`SdkError<PutBucketTaggingError>`](crate::error::PutBucketTaggingError)
    pub fn put_bucket_tagging(&self) -> crate::client::fluent_builders::PutBucketTagging {
                                crate::client::fluent_builders::PutBucketTagging::new(self.handle.clone())
                            }
}

