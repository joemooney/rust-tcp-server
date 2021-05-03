# rust-tcp-server

This is a TCP server example.

It will listen on port 3333.
On the client side we serialize a struct, encode it using bincode crate and send it to the server.
On the server side we deserialize to a byte array and deserialize into a struct.

To write to a TCP port in Linux you can use this handy trick: `echo "foo" > /dev/tcp/localhost/3333`


# Adding to GitHub
`hub create` will create a repo of the same name under your account in GitHub.
`hub` is being replaced by a official new command line tool but I had issues on Linux with that.

