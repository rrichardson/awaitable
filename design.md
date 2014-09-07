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

