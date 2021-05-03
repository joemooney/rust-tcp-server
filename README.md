# rust-tcp-server

This is a TCP server example.

It will list on port 3333.
On the client side we serialize a struct, encode it using bincode crate and send it to the server.
On the server side we deserialize to a byte array and deserialize into a struct.

To test you can `echo "foo" > /dev/tcp/localhost/3333`

