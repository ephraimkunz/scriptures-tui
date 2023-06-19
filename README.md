# scriptures-tui
A terminal user interface for reading the scriptures: The Book of Mormon, Doctrine and Covenants, Pearl of Great Price, and King James version of the Bible.

## Structure

```
src/
├── app.rs     -> holds the state and application logic
├── event.rs   -> handles the terminal events (key press, mouse click, resize, etc.)
├── handler.rs -> handles the key press events and updates the application
├── lib.rs     -> module definitions
├── main.rs    -> entry-point
├── tui.rs     -> initializes/exits the terminal interface
└── ui.rs      -> renders the widgets / UI
```