use crate::error::{Result, VideoClipError};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use tracing::{debug, info};

const GROQ_API_BASE: &str = "https://api.groq.com/openai/v1";
const WHISPER_MODEL: &str = "whisper-large-v3";
const LLM_MODEL: &str = "llama-3.3-70b-versatile";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscriptSegment {
    pub id: u32,
    pub start: f64,
    pub end: f64,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transcript {
    pub text: String,
    #[serde(default)]
    pub segments: Vec<TranscriptSegment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Highlight {
    pub start: f64,
    pub end: f64,
    pub title: String,
    pub reason: String,
}

pub struct AIAnalyzer {
    api_key: String,
    client: reqwest::Client,
}

impl AIAnalyzer {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .unwrap();

        Self { api_key, client }
    }

    pub async fn transcribe(&self, audio_path: &Path) -> Result<Transcript> {
        info!("🎤 Transcribing audio with Groq Whisper...");
        
        let file = tokio::fs::read(audio_path).await?;
        let filename = audio_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("audio.mp3");

        let part = multipart::Part::bytes(file)
            .file_name(filename.to_string())
            .mime_str("audio/mpeg")
            .map_err(|e| VideoClipError::Api(format!("Failed to create multipart: {}", e)))?;

        let form = multipart::Form::new()
            .part("file", part)
            .text("model", WHISPER_MODEL)
            .text("response_format", "verbose_json")
            .text("timestamp_granularities[]", "segment");

        let response = self
            .client
            .post(&format!("{}/audio/transcriptions", GROQ_API_BASE))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(VideoClipError::Api(format!(
                "Whisper API failed with status {}: {}",
                status, error_text
            )));
        }

        let transcript: Transcript = response.json().await?;
        debug!("Transcription complete: {} characters", transcript.text.len());

        Ok(transcript)
    }

    pub async fn find_highlights(
        &self,
        transcript: &Transcript,
        min_duration: u32,
        max_duration: u32,
        num_clips: u32,
    ) -> Result<Vec<Highlight>> {
        info!("🤖 AI analyzing transcript for highlights...");

        let prompt = self.build_analysis_prompt(transcript, min_duration, max_duration, num_clips);

        #[derive(Serialize)]
        struct ChatRequest {
            model: String,
            messages: Vec<Message>,
            temperature: f32,
            response_format: ResponseFormat,
        }

        #[derive(Serialize)]
        struct Message {
            role: String,
            content: String,
        }

        #[derive(Serialize)]
        struct ResponseFormat {
            #[serde(rename = "type")]
            format_type: String,
        }

        #[derive(Deserialize)]
        struct ChatResponse {
            choices: Vec<Choice>,
        }

        #[derive(Deserialize)]
        struct Choice {
            message: ResponseMessage,
        }

        #[derive(Deserialize)]
        struct ResponseMessage {
            content: String,
        }

        let request = ChatRequest {
            model: LLM_MODEL.to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You are a content editor analyzing video transcripts for viral clip potential. Always respond with valid JSON.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: 0.7,
            response_format: ResponseFormat {
                format_type: "json_object".to_string(),
            },
        };

        let response = self
            .client
            .post(&format!("{}/chat/completions", GROQ_API_BASE))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(VideoClipError::Api(format!(
                "LLM API failed with status {}: {}",
                status, error_text
            )));
        }

        let chat_response: ChatResponse = response.json().await?;
        let content = &chat_response.choices[0].message.content;

        debug!("AI response: {}", content);

        // Parse the JSON response
        let highlights = self.parse_highlights_response(content)?;

        if highlights.is_empty() {
            return Err(VideoClipError::NoHighlightsFound);
        }

        info!("✅ Found {} highlights", highlights.len());
        Ok(highlights)
    }

    fn build_analysis_prompt(&self, transcript: &Transcript, min_duration: u32, max_duration: u32, num_clips: u32) -> String {
        format!(
            r#"Analyze the following video transcript and identify the {} most engaging segments for viral short-form content.

Find segments that are:
- Self-contained (make sense without prior context)
- Emotionally engaging (funny, surprising, insightful, dramatic)
- Between {} and {} seconds in duration
- Have a clear hook in the first 3 seconds
- Peak in engagement (climax, punchline, revelation, etc.)

Return a JSON object with a "highlights" array containing objects with these fields:
- start: timestamp in seconds (float)
- end: timestamp in seconds (float)
- title: catchy title for the clip (max 50 chars)
- reason: brief explanation of why this is engaging (max 100 chars)

TRANSCRIPT:
{}

Remember to return ONLY valid JSON with a "highlights" array."#,
            num_clips,
            min_duration,
            max_duration,
            self.format_transcript_for_prompt(transcript)
        )
    }

    fn format_transcript_for_prompt(&self, transcript: &Transcript) -> String {
        if transcript.segments.is_empty() {
            return transcript.text.clone();
        }

        transcript
            .segments
            .iter()
            .map(|seg| {
                format!(
                    "[{:.1}s - {:.1}s] {}",
                    seg.start, seg.end, seg.text
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn parse_highlights_response(&self, content: &str) -> Result<Vec<Highlight>> {
        #[derive(Deserialize)]
        struct HighlightsResponse {
            highlights: Vec<Highlight>,
        }

        let response: HighlightsResponse = serde_json::from_str(content)
            .map_err(|e| {
                VideoClipError::Api(format!(
                    "Failed to parse AI response as JSON: {}. Response was: {}",
                    e, content
                ))
            })?;

        Ok(response.highlights)
    }
}
