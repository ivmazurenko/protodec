# Protobuf Message Decoder

A graphical tool to parse and analyze [Google Protobuf][] messages without knowing their definition.

Instead of trying to predict internal structure of the binary message, this tool allows to select the type of nested data from the ui. Anyway, if any chunk of data can not be decoded, user will see the raw binary data of a field. 

The [Iced][] framework is used to draw UI.

#### Example screencast:

![Main screencast](./screencast.gif)

#### Usage:

Build with a cargo as usual. Currently you can pass the binary file as argument parameter:

```
$ ./protodec serialized_proto_object
```

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
[Iced]: https://github.com/hecrj/iced