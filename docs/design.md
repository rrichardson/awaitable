## Awaitable  -  Asynchronous Promise and Event API


### Introduction

Awaitable is intended to be a high level, low noise API for managing arbitrary events, including asynchronous IO.  By intent, it lacks many of the knobs that one would find on lower level, systems APIs, but that by no means means it should not be performant. 

At a high level, it draws inspiration from both NodeJS's net and file APIs,  and C#'s *Async APIs.  

### Low Level Handle API
At the lowest level the API would be a trait (Awaitable, described later) whose members contain callback registrars which are the primary data access point for notification: 

    trait Awaitable<T> {
        fn on(&self, event: EventType, handler: fn(error: Option<ErrType>, data: T) -> Control),
    
        fn promise(&self, event: EventType) -> Promise,

        fn emit(&self, event: EventType, data: &T)
    ... 

Presumably, IO based Awaitables, such as socket and pipe would emit events on readiness, but they are welcome to do whichever. 

### Mid Level Handle API
There would be convenience functions which wrap the low level Event API and offer the standard IO interface, albeit async only...

        ReadAsync(&self, handler: fn(error: Option<ErrType>, data: &[u8]) -> Control)

        WriteAsync(&self, handler: fn(error: Option<ErrType>, data: &[u8]) -> Control)

Both functions are readiness based, so they would be called then the handle upon which they are waiting is ready to be read/written, or in the case of an error. 

### Promises

In addition, variations of this API would be a Promise based API, which wraps the above functions and presents an interface such as: 

   Read(&self) -> Promise

Where Promise is defined as something like: 

    trait Promise {
         then(handler: fn(data: &[u8]) -> Control) -> Promise;
         error(handler: fn(err: ErrType) -> Control) -> Promise;
    }


There are two unknowns,  The exact meaning and sematics of Control for both Async and Promise functions. 

Whether Promise should be readiness based, or completion based.  My hunch is that making this completion based would simplify the code that the user would have to implement.  

One could achieve both by overriding Promise::data which would be called for every readiness event, whereas then is only called when the requested operation completes successfully.  This would require the user to inherit their IOPromise of choice and implement an alternate data function. 

### Awaitable Implementation

The implementation of on, emit, and promise would be provided.  ReadAsync and WriteAsync and others could be implemented as well, as long as the user emit()'s the appropriate Events in response to data completion. 

The async IO based implementations of Awaitable would likely need to leverage a Reactor or some other external event loop (Timer) 

### High Level Reactor

A reactor which could conveniently serve this callback-per-event model would be a bit higher level than the standard Selector scheme.  It would need to associate an EventType and Awaitable object with its own platform specific event model.  It would also need to be able to handle things like re-registration, as in a user calls socket.ReadAsync(...) and then later on in the same codepath calls socket.WriteAsync. At this point the reactor would have to re-register the same fd but with the additional event to be tracked.  This is fairly trivial for the read/write case, but there are other events to be monitored as well. 

This can all be accomplished without a central lookup table by passing the pointer to the Awaitable object as the token to be passed to the Selector on an event.  The Awaitable object would know which events upon which it is waiting. It would also know how to translate the platform specific events to its own events through a translation mechanism provided by the Reactor layer. 

The Awaitable object itself would have to maintain an event -> callback-list mapping.  A default implementation will be supplied that can be composed in to the awaitable. 

I am assuming that consumers of the Awaitable object simply won't care about the reactor, and therefore the default construction of an IO Awaitable will be a singleton IO Reactor that is spun up on first access.  There will be the option of an alternate construction mechanism which will take a new Reactor and set it running in its own thread. 

It is the responsibility of the Reactor to ensure that controls into the reactor (registration, run/stop, etc) are managed in a thread-safe way. 

### Async/Await 

The ability to invert callback control-flow into something that appears to be blocking calls is desireable. With green-threads not living up to their promise, a lightweight mechanism is needed. 

It is the intent of this module to provide macros named async and await, which will construct a state machine to manage events, and provide markers into event states, respectively. 

It is hoped that the code will appear as follows: 

    asyncfn!(test_tcp (hostport: string) -> Promise<string> {
        //TcpClient.connect normally returns a promise
        // but this await! call will make it appear as
        // though it is a blocking call and returning the value
        let client = await!(TcpClient.connect());
        len num = await!(client.send("Ping!");
        let mut buf = [u8, ..5];
        let result = await!(client.receive(&buf);
        if result == "Pong!" {
            println!("Success!");
        }
    })

Immediate code and async code can be executed side by side in an async! function, but the asynchronous calls that must be blocked have to be wrapped in await!. 

Something like  

    let result = await!(getNumberAsync("A")) + await!(getNumberAsync("B")); 

should be no problem. 

await! is demonstrated above taking a single promise, but I would also like to introduce functions  

awaitAll! and awaitAny! which would take an array of promises and in the first case, block until all promises have been fulfilled, in the Any case, it would block until one was fulfilled.  

The possibilities here extend quite far, the system could leverage a Promise management library similar to bluebird which can do a great number of things against Promises, such as cascade, or map/fold against an accumulator. 