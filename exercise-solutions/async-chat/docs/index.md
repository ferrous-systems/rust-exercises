# Tutorial: Writing a chat

Nothing is simpler than creating a chat server, right?
Not quite, chat servers expose you to all the fun of asynchronous programming:

How will the server handle clients connecting concurrently?

How will it handle them disconnecting?

How will it distribute the messages?

This tutorial explains how to write a chat server in `tokio`.
