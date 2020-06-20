# Protobuf Message Decoder

A graphical tool to parse and analyze [Google Protobuf][] messages without knowing their definition.

Instead of trying to predict internal structure of the binary message, this tool allows to select the type of nested data from the ui. Anyway, if any chunk of data can not be decoded, user will see the raw binary data of a field. 

The [Seed] with [newcss][] framework are used to draw UI.

For now this program it is not fully finished. Any suggestions and PR are welcome.

#### Plans to the future:

1. Packed repeated elements are not implemented yet.
2. Groups are not implemented yet.
3. UI improvements, scroll bars, pretty looking, etc.
4. Support the native file dialog to open the binary from ui.

#### Similar tools

[protobuf-inspector][] - command line tool, written on the python

[protobuf-inspector]: https://github.com/mildsunrise/protobuf-inspector
[Google Protobuf]: https://developers.google.com/protocol-buffers
[new.css]: https://newcss.net
[Seed]: https://github.com/seed-rs/seed