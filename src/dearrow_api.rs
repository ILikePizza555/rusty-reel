use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeArrowApiError {
    #[error("error extracting a videoID from `{0}` (not a valid youtube url or videoID)")]
    VideoIdParsingError(String)
}

pub struct VideoId<'a> (&'a str);

impl <'a> TryFrom<&str> for VideoId<'a> {
    type Error = DeArrowApiError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Err(DeArrowApiError::VideoIdParsingError(value.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::{DeArrowApiError, VideoId};

    #[test]
    fn full_youtube_url_into_video_id() -> Result<(), DeArrowApiError> {
        VideoId::try_from("https://www.youtube.com/watch?v=oBnCgu7bdQk").and(Ok(()))
    }

    fn short_youtube_url_into_video_id() -> Result<(), DeArrowApiError> {
        VideoId::try_from("https://youtu.be/_u6f9beKbwg?si=cQn5mAT_Q5pusqRy").and(Ok(()))
    }
}