// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`StartSpeechSynthesisTask`](crate::client::fluent_builders::StartSpeechSynthesisTask) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`engine(Engine)`](crate::client::fluent_builders::StartSpeechSynthesisTask::engine) / [`set_engine(Option<Engine>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::set_engine): <p>Specifies the engine (<code>standard</code> or <code>neural</code>) for Amazon Polly to use when processing input text for speech synthesis. Using a voice that is not supported for the engine selected will result in an error.</p>
    ///   - [`language_code(LanguageCode)`](crate::client::fluent_builders::StartSpeechSynthesisTask::language_code) / [`set_language_code(Option<LanguageCode>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::set_language_code): <p>Optional language code for the Speech Synthesis request. This is only necessary if using a bilingual voice, such as Aditi, which can be used for either Indian English (en-IN) or Hindi (hi-IN). </p>  <p>If a bilingual voice is used and no language code is specified, Amazon Polly uses the default language of the bilingual voice. The default language for any voice is the one returned by the <a href="https://docs.aws.amazon.com/polly/latest/dg/API_DescribeVoices.html">DescribeVoices</a> operation for the <code>LanguageCode</code> parameter. For example, if no language code is specified, Aditi will use Indian English rather than Hindi.</p>
    ///   - [`lexicon_names(Vec<String>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::lexicon_names) / [`set_lexicon_names(Option<Vec<String>>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::set_lexicon_names): <p>List of one or more pronunciation lexicon names you want the service to apply during synthesis. Lexicons are applied only if the language of the lexicon is the same as the language of the voice. </p>
    ///   - [`output_format(OutputFormat)`](crate::client::fluent_builders::StartSpeechSynthesisTask::output_format) / [`set_output_format(Option<OutputFormat>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::set_output_format): <p>The format in which the returned output will be encoded. For audio stream, this will be mp3, ogg_vorbis, or pcm. For speech marks, this will be json. </p>
    ///   - [`output_s3_bucket_name(impl Into<String>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::output_s3_bucket_name) / [`set_output_s3_bucket_name(Option<String>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::set_output_s3_bucket_name): <p>Amazon S3 bucket name to which the output file will be saved.</p>
    ///   - [`output_s3_key_prefix(impl Into<String>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::output_s3_key_prefix) / [`set_output_s3_key_prefix(Option<String>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::set_output_s3_key_prefix): <p>The Amazon S3 key prefix for the output speech file.</p>
    ///   - [`sample_rate(impl Into<String>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::sample_rate) / [`set_sample_rate(Option<String>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::set_sample_rate): <p>The audio frequency specified in Hz.</p>  <p>The valid values for mp3 and ogg_vorbis are "8000", "16000", "22050", and "24000". The default value for standard voices is "22050". The default value for neural voices is "24000".</p>  <p>Valid values for pcm are "8000" and "16000" The default value is "16000". </p>
    ///   - [`sns_topic_arn(impl Into<String>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::sns_topic_arn) / [`set_sns_topic_arn(Option<String>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::set_sns_topic_arn): <p>ARN for the SNS topic optionally used for providing status notification for a speech synthesis task.</p>
    ///   - [`speech_mark_types(Vec<SpeechMarkType>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::speech_mark_types) / [`set_speech_mark_types(Option<Vec<SpeechMarkType>>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::set_speech_mark_types): <p>The type of speech marks returned for the input text.</p>
    ///   - [`text(impl Into<String>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::text) / [`set_text(Option<String>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::set_text): <p>The input text to synthesize. If you specify ssml as the TextType, follow the SSML format for the input text. </p>
    ///   - [`text_type(TextType)`](crate::client::fluent_builders::StartSpeechSynthesisTask::text_type) / [`set_text_type(Option<TextType>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::set_text_type): <p>Specifies whether the input text is plain text or SSML. The default value is plain text. </p>
    ///   - [`voice_id(VoiceId)`](crate::client::fluent_builders::StartSpeechSynthesisTask::voice_id) / [`set_voice_id(Option<VoiceId>)`](crate::client::fluent_builders::StartSpeechSynthesisTask::set_voice_id): <p>Voice ID to use for the synthesis. </p>
                            /// - On success, responds with [`StartSpeechSynthesisTaskOutput`](crate::output::StartSpeechSynthesisTaskOutput) with field(s):
    ///   - [`synthesis_task(Option<SynthesisTask>)`](crate::output::StartSpeechSynthesisTaskOutput::synthesis_task): <p>SynthesisTask object that provides information and attributes about a newly submitted speech synthesis task.</p>
                            /// - On failure, responds with [`SdkError<StartSpeechSynthesisTaskError>`](crate::error::StartSpeechSynthesisTaskError)
    pub fn start_speech_synthesis_task(&self) -> crate::client::fluent_builders::StartSpeechSynthesisTask {
                                crate::client::fluent_builders::StartSpeechSynthesisTask::new(self.handle.clone())
                            }
}

