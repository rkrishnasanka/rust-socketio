# TODO List:

ServerBuilder:
- [ ] Check if the `allow_request` function has the right signature (success / error codes)
- [ ]  Figure out what the trasports vector should be a string or some kind of vector types
- [ ]  Figure out how to integrate the websocket server
- [ ]  Figure out if the return types for `ServerBuilder` methods `handle_request` and `handle_upgrade` are correct. Currently they are returning self to allow for a chaining pattern but this might not be the best way to do it in rust.
- [ ]  Identify the right backbone servers for http and websocket to allow the engineio server to just wrapper them and not have to implement them from scratch.
- [ ]  Create sample code for how things are supposed to be initialized and used. This will help with the API design and also help with the development.
- [ ]  Figure out if we need to have a callback argument for all the functions. Does that go against pattern/idioms of rust?



