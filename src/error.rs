use thiserror::Error;

pub type Result<T> = std::result::Result<T, TimeFloError>;

#[derive(Error, Debug)]
pub enum TimeFloError {
    #[error("i/o error")]
    Io(#[from] std::io::Error),
    #[cfg(feature = "notifications")]
    #[error("notification error")]
    Notification(#[from] notify_rust::error::Error),
    #[cfg(feature = "sound")]
    #[error("sound decoder error")]
    SoundDecoder(#[from] rodio::decoder::DecoderError),
    #[cfg(feature = "sound")]
    #[error("sound playback error")]
    SoundPlayback(#[from] rodio::PlayError),
}
