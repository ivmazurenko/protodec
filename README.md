# Protobuf Message Decoder

[Open application][]

A online graphical tool to parse and analyze [Google Protobuf][] messages without knowing their definition.

Instead of trying to predict internal structure of the binary message, this tool allows to select the type of nested data from the ui. Anyway, if any chunk of data can not be decoded, user will see the raw binary data of a field. 

The [Seed] with [new.css][] framework are used to draw UI.

For now this program it is not fully finished. Any suggestions and PR are welcome.

#### Plans to the future:

1. Packed repeated elements are not implemented yet.
2. Groups are not implemented yet.
3. UI improvements, scroll bars, pretty looking, etc.
4. Support the file dialog to open the binary from ui.
5. Support hex data on the input field.

#### Similar tools

[protobuf-inspector][] - command line tool, written on the python

[protobuf-decoder][] - gui tool, written on the python

#### Support

Please, for any questions open issue on the github or send me the message directly to the [telegram][].

[protobuf-inspector]: https://github.com/mildsunrise/protobuf-inspector
[Google Protobuf]: https://developers.google.com/protocol-buffers
[new.css]: https://newcss.net
[Seed]: https://github.com/seed-rs/seed
[Open application]: https://ivmazurenko.github.io/protodec/
[protobuf-decoder]: https://github.com/nevermoe/protobuf-decoder
[telegram]: https://telegram.me/imazurenko