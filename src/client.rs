//! A client connection to the VLC interface.
//!
//! ## Types
//!
//! ### Connection types:
//!
//! * [`Client`] - Represents a connection to VLC's TCP interface.
//!
//! ### Media types:
//!
//! * [`Track`] - Represents a media track in a VLC player's playlist.
//! * [`Playlist`] - A collection of tracks.
//! * [`Subtitle`] - A subtitle track associated with a media file.
//! * [`Subtitles`] - A collection of subtitle tracks.
//!
//! When using the library, you'd typically construct a new [`Client`] and then proceed to issue commands by using the client's methods.

mod media;
mod socket;

pub use media::Playlist;
pub use media::Subtitle;
pub use media::Subtitles;
pub use media::Track;
pub use media::MAX_VOLUME;
pub use media::MIN_VOLUME;

use std::io::prelude::*;
use std::net::ToSocketAddrs;

use crate::Result;

use media::FromParts;
use socket::IoSocket;
use socket::PROMPT;

/// A connection to a VLC player's TCP interface.
pub struct Client {
    socket: IoSocket,
}

impl Client {
    /// Establishes a connection to a VLC player's TCP interface at the given address.
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let player = Client::connect("127.0.0.1:9090").unwrap();
    /// ```
    pub fn connect<A>(addr: A) -> Result<Client>
    where
        A: ToSocketAddrs,
    {
        Ok(Self { socket: IoSocket::connect(addr)? })
    }

    /// Gets a list of tracks in the VLC player's playlist.
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// let playlist = player.playlist().unwrap();
    /// for track in playlist {
    ///     println!("{}", track);
    /// }
    /// ```
    pub fn playlist(&mut self) -> Result<Playlist> {
        writeln!(self.socket, "playlist")?;
        self.socket.flush()?;

        let mut buf = Vec::new();
        self.socket.read_until(PROMPT, &mut buf)?;

        let out = String::from_utf8_lossy(&buf);

        Ok(out.lines().filter_map(Track::from_parts).collect())
    }

    /// Gets a list of subtitle tracks for the current media file.
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// let subtitles = player.subtitles().unwrap();
    /// for strack in subtitles {
    ///     println!("{}", strack);
    /// }
    /// ```
    pub fn subtitles(&mut self) -> Result<Subtitles> {
        writeln!(self.socket, "strack")?;
        self.socket.flush()?;

        let mut buf = Vec::new();
        self.socket.read_until(PROMPT, &mut buf)?;

        let out = String::from_utf8_lossy(&buf);

        Ok(out.lines().filter_map(Subtitle::from_parts).collect())
    }

    /// Gets the VLC player's current volume.
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// let volume = player.get_volume().unwrap();
    /// println!("the current volume is {}", volume);
    /// ```
    pub fn get_volume(&mut self) -> Result<u8> {
        writeln!(self.socket, "volume")?;
        self.socket.flush()?;

        let mut line = String::new();
        self.socket.read_line(&mut line)?;

        let volume = line.trim().parse::<u16>()?;

        if volume <= (MAX_VOLUME as u16) {
            Ok(volume as u8)
        } else {
            Ok(MAX_VOLUME)
        }
    }

    /// Sets the VLC player's volume to the given amount.
    ///
    /// If `amt` is greater than [`MAX_VOLUME`], it defaults to the max volume.
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// player.set_volume(50).unwrap();
    /// assert_eq!(player.get_volume().unwrap(), 50);
    /// ```
    pub fn set_volume(&mut self, mut amt: u8) -> Result<()> {
        if amt > MAX_VOLUME {
            amt = MAX_VOLUME;
        }

        // Spam the interface until we get the desired output.
        while self.get_volume()? != amt {
            writeln!(self.socket, "volume {}", amt)?;
            self.socket.flush()?;
        }
        Ok(())
    }

    /// Returns whether or not the current media track is playing.
    ///
    /// Note that if the track is paused, the method still returns `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// let is_playing = player.is_playing().unwrap();
    /// if is_playing {
    ///     println!("the track is currently playing!");
    /// } else {
    ///     println!("the track is currently stopped!");
    /// }
    /// ```
    pub fn is_playing(&mut self) -> Result<bool> {
        writeln!(self.socket, "is_playing")?;
        self.socket.flush()?;

        let mut line = String::new();
        self.socket.read_line(&mut line)?;

        Ok(line.trim() == "1")
    }

    /// Plays the current media track.
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// player.play().unwrap();
    /// assert_eq!(player.is_playing().unwrap(), true);
    /// ```
    pub fn play(&mut self) -> Result<()> {
        // Only issue the 'play' command if the playlist is not empty.
        if !self.playlist()?.is_empty() {
            // Spam the interface until we get the desired output.
            while !self.is_playing()? {
                writeln!(self.socket, "play")?;
                self.socket.flush()?;
            }
        }
        Ok(())
    }

    /// Stops the current media track's playback.
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// player.stop().unwrap();
    /// assert_eq!(player.is_playing().unwrap(), false);
    /// ```
    pub fn stop(&mut self) -> Result<()> {
        // Spam the interface until we get the desired output.
        while self.is_playing()? {
            writeln!(self.socket, "stop")?;
            self.socket.flush()?;
        }
        Ok(())
    }

    /// Pauses the current track's playback.
    ///
    /// Does nothing if the track is stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// player.pause().unwrap();
    /// ```
    pub fn pause(&mut self) -> Result<()> {
        if self.is_playing()? {
            // The 'pause' command works as a toggle, so we need to ensure that the track is playing before we execute it to get the desired behavior.
            writeln!(self.socket, "play")?;
            writeln!(self.socket, "pause")?;
            self.socket.flush()?;
        }
        Ok(())
    }

    /// Gets the elapsed time since the track's beginning (in seconds).
    ///
    /// Returns `None` if the current track is stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// let seconds = player.get_time().unwrap();
    /// ```
    pub fn get_time(&mut self) -> Result<Option<u32>> {
        writeln!(self.socket, "get_time")?;
        self.socket.flush()?;

        let mut line = String::new();
        self.socket.read_line(&mut line)?;

        Ok(line.trim().parse().ok())
    }

    /// Moves the track's playback forward by the given amount (in seconds).
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// player.forward(5).unwrap();
    /// ```
    pub fn forward(&mut self, secs: u32) -> Result<()> {
        writeln!(self.socket, "seek +{}", secs)?;
        self.socket.flush()?;

        Ok(())
    }

    /// Moves the track's playback backward by the given amount (in seconds).
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// player.rewind(5).unwrap();
    /// ```
    pub fn rewind(&mut self, secs: u32) -> Result<()> {
        writeln!(self.socket, "seek -{}", secs)?;
        self.socket.flush()?;

        Ok(())
    }

    /// Gets the current media track's title.
    ///
    /// Returns `None` if the media player is stopped.
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// let current_track = player.get_title().unwrap();
    /// if let Some(title) = current_track {
    ///     println!("the track '{}' is currently playing!", title);
    /// }
    /// ```
    pub fn get_title(&mut self) -> Result<Option<String>> {
        writeln!(self.socket, "get_title")?;
        self.socket.flush()?;

        let mut line = String::new();
        self.socket.read_line(&mut line)?;

        // If the line is empty, it means that the player is currently stopped - so we can just return `None`.
        if line.trim().len() != 0 {
            Ok(Some(line.trim().to_owned()))
        } else {
            Ok(None)
        }
    }

    /// Plays the next track in the playlist.
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// player.next().unwrap();
    /// ```
    pub fn next(&mut self) -> Result<()> {
        writeln!(self.socket, "next")?;
        self.socket.flush()?;

        Ok(())
    }

    /// Plays the previous track in the playlist.
    ///
    /// # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// player.prev().unwrap();
    /// ```
    pub fn prev(&mut self) -> Result<()> {
        writeln!(self.socket, "prev")?;
        self.socket.flush()?;

        Ok(())
    }

    /// Toggles the media player's fullscreen mode on/off.
    ///
    ///  # Examples
    ///
    /// ```
    /// use vlc_rc::Client;
    ///
    /// let mut player = Client::connect("127.0.0.1:9090").unwrap();
    ///
    /// player.fullscreen(true).unwrap();
    /// println!("fullscreen is on!");
    ///
    /// player.fullscreen(false).unwrap();
    /// println!("fullscreen is off!");
    /// ```
    pub fn fullscreen(&mut self, on: bool) -> Result<()> {
        writeln!(self.socket, "fullscreen {}", if on { "on" } else { "off" })?;
        self.socket.flush()?;

        Ok(())
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if let Ok(_) = self.socket.shutdown() {}
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use super::Client;
    use super::Result;

    fn connect() -> Result<Client> {
        let addr = env::var("TEST_ADDR")
            .expect("use the 'test.sh' bash script to run tests!");

        Client::connect(addr)
    }

    #[test]
    fn get_and_set_volume() -> Result<()> {
        let mut client = connect()?;

        client.set_volume(25)?;
        assert_eq!(client.get_volume()?, 25);

        client.set_volume(0)?;
        assert_eq!(client.get_volume()?, 0);

        Ok(())
    }

    #[test]
    fn play_and_stop() -> Result<()> {
        let mut client = connect()?;

        client.play()?;
        assert_eq!(client.is_playing()?, true);

        client.stop()?;
        assert_eq!(client.is_playing()?, false);

        Ok(())
    }

    #[test]
    fn forward() -> Result<()> {
        let mut client = connect()?;

        client.pause()?;

        let before = match client.get_time()? {
            Some(t) => t,
            _ => return Ok(()),
        };

        client.forward(5)?;

        let after = match client.get_time()? {
            Some(t) => t,
            _ => return Ok(()),
        };

        assert_eq!(after, before + 5);

        Ok(())
    }

    #[test]
    fn rewind() -> Result<()> {
        let mut client = connect()?;

        client.forward(10)?;

        let before = match client.get_time()? {
            Some(t) => t,
            _ => return Ok(()),
        };

        client.rewind(5)?;

        let after = match client.get_time()? {
            Some(t) => t,
            _ => return Ok(()),
        };

        assert_eq!(after, (before).checked_sub(5).unwrap_or(0));

        Ok(())
    }
}
