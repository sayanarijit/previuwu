# previuwu

A generic previewer that can preview anything as everything.

Uses [egui][3] to render the preview window.

STATUS: Proof of Concept (⚠️ heavy work in progress).

https://user-images.githubusercontent.com/11632726/180646344-4cd4e487-9dd9-49a7-9e90-a1baac291124.mp4

### Install

Better don't install it as of now, unless you want to contribute.

```bash
cargo install --git https://github.com/sayanarijit/previuwu
```

### Usage

Preview a single file:

```bash
previuwu /path/to/file
```

Allow streaming input from stdin:

```bash
previuwu /path/to/file --pipe -
```

Also allow streaming input from a named pipe[1]:

```bash
# mkfifo path/to/input.fifo
previuwu /path/to/file --pipe - --pipe path/to/input.fifo
```

### Use Case

Example usage with [xplr][2]:

```bash
# Create a fifo file
mkfifo /tmp/xplr.fifo

# Run previuwu in background (will close automatically when done)
previuwu --pipe /tmp/xplr.fifo &

# Run xplr with fifo enabled
xplr --on-load 'StartFifo: /tmp/xplr.fifo'
```

### Supports

I plan to support as many input types as possible.

- [x] Stdin
- [x] Named Pipes (fifo)
- [ ] Socket
- ...ideas?

Very basic (ugly) support for the following file types has been implemented:

- [x] Directory
- [x] Text
- [x] Images
- ...contribute?

### Contribute

First of all, thank you.

Please go through `src/preview.rs` and try to implement laoding and/or rendering as many file types as you can.

Parsing is the easy part. Rendering might require some experience with [egui][3].

Some files can be really slow to load/render in development mode (`cargo run`). Try running in release mode (`cargo run --release`).

[1]: https://man7.org/linux/man-pages/man7/fifo.7.html
[2]: https://xplr.dev
[3]: https://github.com/emilk/egui