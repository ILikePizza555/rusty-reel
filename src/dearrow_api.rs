use log::{debug, info};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use uuid::Uuid;

static DEARROW_BRANDING_API: &'static str = "https://sponsor.ajay.app/api/branding";

#[derive(Error, Debug)]
pub enum DeArrowApiError {
    #[error("Error: `{0}` is not a valid youtube url or video ID")]
    VideoIdParseError(String),
    #[error("Error sending request")]
    ReqwestError(#[from] reqwest::Error)
}

#[derive(Serialize)]
pub struct VideoId<'a> (&'a str);

#[derive(Deserialize)]
pub struct TitleResponse {
    title: String,
    original: bool,
    votes: u64,
    locked: bool,
    uuid: Uuid
}

#[derive(Deserialize)]
pub struct ThumbnailResponse {
    timestamp: Option<u64>,
    original: bool,
    votes: u64,
    locked: bool,
    uuid: Uuid
}

#[derive(Deserialize)]
pub struct BrandingResponse {
    titles: Vec<TitleResponse>,
    thumbnails: Vec<ThumbnailResponse>,
    random_time: u64,
    video_duration: Option<u64>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BrandingRequest<'a> {
    video_id: VideoId<'a>,
    service: Option<String>,
    return_user_id: bool
}

impl <'a> BrandingRequest<'a> {
    fn new(video_id: VideoId<'a>) -> Self {
        BrandingRequest {
            video_id,
            service: None,
            return_user_id: false
        }
    }

    async fn send(&self, client: &Client) -> Result<BrandingResponse, DeArrowApiError> {
        Ok(client.get(DEARROW_BRANDING_API)
            .json(self)
            .send()
            .await?
            .json::<BrandingResponse>()
            .await?)
    }
}

impl <'a> TryFrom<&'a str> for VideoId<'a> {
    type Error = DeArrowApiError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split('/')
            .filter(|section| !section.is_empty())
            .collect();

        debug!(target: "dearrow_api", "Split url into: {:#?}", split);

        match split.as_slice() {
            [protocol, "www.youtube.com", path] if protocol.starts_with("http") => {
                info!(target: "dearrow_api", "Parsing as full youtube url syntax.");

                path.strip_prefix("watch?v=")
                    .and_then(|s| s.get(..11))
                    .map(Self)
                    .ok_or_else(|| DeArrowApiError::VideoIdParseError(value.to_owned()))
            }
            [protocol, "youtu.be", path] if protocol.starts_with("http") => {
                info!(target: "dearrow_api", "Parsing as short youtube url syntax.");
                
                path.get(..11)
                    .map(Self)
                    .ok_or_else(|| DeArrowApiError::VideoIdParseError(value.to_owned()))
            }
            [video_id] if video_id.len() == 11 => {
                info!(target: "dearrow_api", "Parsing as plain video id.");

                Ok(Self(video_id))
            }
            _ => Err(DeArrowApiError::VideoIdParseError(value.to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DeArrowApiError, VideoId};

    #[test]
    fn full_youtube_url_into_video_id() -> Result<(), DeArrowApiError> {
        VideoId::try_from("https://www.youtube.com/watch?v=oBnCgu7bdQk").and(Ok(()))
    }

    #[test]
    fn short_youtube_url_into_video_id() -> Result<(), DeArrowApiError> {
        VideoId::try_from("https://youtu.be/_u6f9beKbwg?si=cQn5mAT_Q5pusqRy").and(Ok(()))
    }
}