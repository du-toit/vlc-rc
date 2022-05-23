<br />
<div align="center">
    <h1>vlc-rc</h1>
    <p>A rust library used to interact with a VLC player's TCP interface.</p>
</div>

## About

This is a (WIP) rust library you can use to interact with [VLC](https://www.videolan.org/vlc) programmatically by using its built-in [TCP interface](https://wiki.videolan.org/Documentation:Advanced_Use_of_VLC/#RC_and_RTCI).

VLC's TCP interface is not well documented, and is *very* unpredictable at times, making it exceedingly hard to test in a deterministic matter. Regardless, the library aims to be as **stable** and **testable** as possible!

## Requirements

* [Rust](https://www.rust-lang.org/)
* [VLC media player](https://www.videolan.org/vlc/)

### Enabling the VLC interface.

There are two ways to enable VLC's TCP interface.

#### Option 1

You can launch VLC with CLI args like so:

```sh
vlc --rc-host 127.0.0.1:9090 # Or any <host>:<port> you prefer!
```

#### Option 2

You can enable it via the GUI and it will run each time you start VLC.

1. Start VLC player.
2. At the top-left toolbar, go to `Tools` -> `Preferences` (Ctrl+P)
3. Enable 'Advanced Settings' by selecting `All` at the bottom left of the preferences window (just under `Show Settings`).
4. Scroll down until you see the `Interface` item and then select `Main interfaces`.
5. Below 'Extra interface modules', check the `Lua interpreter` option.
6. Then to the list on the left, click the arrow next to `Main interfaces` and then select the `Lua` item.
7. At the top, set the `Lua interface` field's value to `rc`.
8. Just below that, set the `Lua interface configuration` field's value to `rc={host='127.0.0.1:9090'}` (or any host/port you prefer).
9. Restart VLC player to start the interface in the background.

## Usage

Add the library as a dependency to [Cargo.toml](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html):

```toml
[dependencies]
vlc-rc = "0.1.0
```

### Example

```rust
use vlc_rc::Client;

let mut client = Client::connect("127.0.0.1:9090")?;

// Set the player's volume.
client.set_volume(25)?;
assert_eq!(client.get_volume()?, 25);

// Stop the track's playback.
client.stop()?;
assert_eq!(client.is_playing()?, false);

// Skip to the next track.
client.next()?;
```

## Contributing

See [CONTRIBUTING](CONTRIBUTING.md).
