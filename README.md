# Log multiplexer

This application multiplexes incoming log messages from concurrent TCP
streams into a single output TCP stream that conveys them to a target
destination, typically a database or other persistent store. The rationale
is that although such a backend database might be capable of handling
the incoming streams itself, they would all be writing to the same table, and will therefore contend for locks. By
performing the multiplexing upstream of the database, this application
spares it that lock contention, and should therefore increase the write throughput.

The database targeted by this application is MongoDB. Future plans include
targeting others.

A log message is assumed to be terminated by
a newline (`\n`) and parseable into JSON. Future plans include
relaxing the requirement of newline termination.

Observe that this application combines two functionalities: it multiplexes incoming streams, and writes the output stream to Mongo. For better separation of concerns, it should be broken into two independent applications, one limited to multiplexing, the
other limited to writing a single incoming stream
to Mongo. The former would thus become a multiplexer
of any kind of messages, not just logs.

## Build, Test, Doc

`Makefile` provides corresponding targets.

## Execution

`Makefile` provides `run` and `release` targets, for
development and production respectively. The following environment
variables are recognized:
* `PORT`: port on which to listen
* `CHANNEL_CAPACITY` (optional): capacity of the multiplexing
  channel. Default: 100.

If provided on the command line, the flag `--listen-locally`
makes the app listen on `127.0.0.1`; otherwise it will listen
on `0.0.0.0`.