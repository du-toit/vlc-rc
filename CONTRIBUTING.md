<br />
<div align="center">
    <h1>Contributing</h1>
</div>

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

## Pull Requests

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Running Tests

You may notice that just running `cargo t` fails almost every test. This happens for a variety of reasons, the most obvious one being that VLC needs to be running to test the library.

We have a little `test.sh` bash script to help us with that.

To run tests:

```sh
bash test.sh
```

### What it does

* First it sets a `TEST_ADDR` environment variable for the duration of the script.
* It runs VLC in the background, playing the `samples/audio.mp3` file (which is just 30 seconds of silence), piping all of VLC's output to `/dev/null` so it does not interfere with the tests' output.
* It runs the doc tests, and then every unit test **individually**.
* After running the tests, the script then terminates VLC.

### Why its needed

Besides starting up VLC for us so we don't have to, the script runs each unit test individually.

The interface is very unpredictable when using it programmatically. For example, the output we receive can be "outdated" even if it is just by a few milliseconds:

```rust
let volume = client.get_volume()?;
assert_eq!(volume, 10);

client.set_volume(5)?;
assert_eq!(client.get_volume()?, 5); // FAILS, get_volume() == 10.
```

Even though we've set the volume to `5`, `get_volume` can still return the previous value of `10`. Running it the first time may fail, while running it a second or third time may succeed. It becomes even more unpredictable when you try testing a bunch of methods all at once!

There are a few hacks in the codebase we can use to minimize this behavior, like `set_volume`'s implementation:

```rust
pub fn set_volume(&mut self, mut amt: u8) -> Result<()> {
    ...
    while self.get_volume()? != amt {
        writeln!(self.socket, "volume {amt}")?;
        self.socket.flush()?;
    }
    Ok(())
}
```

We just repeatedly "spam" the interface until it returns the volume we set it to.

These things help, but it does nothing to mend the tests because different methods interfere with one another.

A simple solve for this is just to run each test individually instead of running all of them all at once.

Tests should succeed at least 90% of the time. Other times, it can fail seemingly randomly because of how unpredictable the interface itself is.