use lazy_static::lazy_static;
use regex::Regex;

/// The minimum amount for a volume setting.
pub const MIN_VOLUME: u8 = 0;

/// The maximum amount for a volume setting.
pub const MAX_VOLUME: u8 = 200;

/// A type alias for a collection of [tracks](Track).
pub type Playlist = Vec<Track>;

/// A type alias for a collection of [subtitles](Subtitle).
pub type Subtitles = Vec<Subtitle>;

/// A trait implemented by types that can be constructed from the VLC interface's output.
pub(crate) trait FromParts: Sized {
    /// Attempts to construct a type from the given VLC output - returning `None` if it is not possible.
    fn from_parts(parts: &str) -> Option<Self>;
}

/// A media track in a VLC player's [playlist](Playlist).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track {
    index: i32,
    title: String,
    length: String,
}

impl Track {
    /// Gets the track's index in the playlist.
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Gets the track's title - commonly the file name.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Gets the track's length as `<hours>:<minutes>:<seconds>`.
    pub fn length(&self) -> &str {
        &self.length
    }
}

impl std::fmt::Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} ({})", self.index, self.title, self.length)
    }
}

impl FromParts for Track {
    fn from_parts(parts: &str) -> Option<Self> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?x)
                \| # List item delimiter.
                \s+
                [\*]?
                (?P<index>[\d]+) # The track's index.
                \s+
                -
                \s+
                (?P<title>.+) # The track's title.
                \s
                \(
                (?P<length>\d\d:\d\d:\d\d) # The track's length.
                .*
        ",
            )
            .unwrap();
        };
        let caps = REGEX.captures(parts)?;
        Some(Self {
            index: caps["index"].parse().ok()?,
            title: caps["title"].to_owned(),
            length: caps["length"].to_owned(),
        })
    }
}

/// A subtitle track associated with a media file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subtitle {
    index: i32,
    title: String,
}

impl std::fmt::Display for Subtitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.index, self.title)
    }
}

impl Subtitle {
    /// Gets the subtitle track's index in VLC.
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Gets the subtitle track's name.
    pub fn title(&self) -> &str {
        &self.title
    }
}

impl FromParts for Subtitle {
    fn from_parts(parts: &str) -> Option<Self> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?x)
                \| # List item delimiter.
                \s+
                (?P<index>[\-]?[\d]+) # The subtitle's index.
                \s+
                -
                \s+
                (?P<title>.+) # The subtitle track's title.
        ",
            )
            .unwrap();
        };

        let caps = REGEX.captures(parts)?;
        Some(Self {
            index: caps["index"].parse().ok()?,
            title: caps["title"].to_owned(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// A helper macro used to test if the input matches the expected output by using the type's [`FromParts`] implementation.
    macro_rules! test_from_parts {
        ($in:expr, $out:expr) => {
            assert_eq!(FromParts::from_parts($in), $out);
        };

        ($t:ident, $in:expr, $out:expr) => {
            assert_eq!($t::from_parts($in), $out);
        };
    }

    #[test]
    fn track_from_parts_none() {
        test_from_parts!(Track, "+----[ Playlist - playlist ]", None);
        test_from_parts!(Track, "| 1 - Playlist", None);
        test_from_parts!(Track, "| 2 - Media Library", None);
        test_from_parts!(Track, "+----[ End of playlist ]", None);
    }

    #[test]
    fn track_from_parts_some() {
        test_from_parts!(
            "| 8 - Chopin Nocturnes.mp3 (01:50:55)",
            Some(Track {
                index: 8,
                title: "Chopin Nocturnes.mp3".into(),
                length: "01:50:55".into()
            })
        );
        test_from_parts!(
            "| *1 - Bach (00:00:01).mp3 (01:50:55)",
            Some(Track {
                index: 1,
                title: "Bach (00:00:01).mp3".into(),
                length: "01:50:55".into()
            })
        );
    }

    #[test]
    fn subtitle_from_parts_none() {
        test_from_parts!(Subtitle, "+----[ spu-es ]", None);
        test_from_parts!(Subtitle, "+----[ end of spu-es ]", None);
    }

    #[test]
    fn subtitle_from_parts_some() {
        test_from_parts!(
            "| -1 - Disable *",
            Some(Subtitle { index: -1, title: "Disable *".into() })
        );
        test_from_parts!(
            "| 2 - Track 1 - [English]",
            Some(Subtitle { index: 2, title: "Track 1 - [English]".into() })
        );
    }
}
