
## The Event System

A generic event registration/alert system has two very tricky requirements: 

1. Allow users of the API to register arbitrary event names. 
2. Interoperate with low level event generation systems such as epoll. 

Alone, this is not difficult, but if you through a third requirement in there 

3. Require minimal space and as few allocations as possible. 
4. Apply strongly typed semantics. 

Now it gets tricky.  If we used a string based event system it would be easy, a person could register an arbitrary event
handler as a string and a callback, and any time that string was emitted, the callback would be called. 

The downsides of strings are: 
1. They are not typesafe. 
2. They can be misspelled with no warning
3. They take up space and are sometimes slow to compare. 


So with that in mind, lets set out to design an event system which is: 
1. Centered around a unique Event type. 
2. Is a closed set (at the end of compile time) 


The best answer I can come up with is based on
[bitflags](http://doc.rust-lang.org/src/std/home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libstd/bitflags.rs.html) 

The difference being that, if we want multiple people to participate in producing events for later use, they must be
able to contribute types of their own. The easiest option will be to provide a hash value for all event names. 
The hash value will be calculated by  hash(struct-name + event-name) so that incidental conflicts can be avoided. 


These hash values can be loaded directly into a Hash table managed by the Awaitable object.
