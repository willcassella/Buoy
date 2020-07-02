# Opportunities for performance improvement

## Ids
Right now a lot of time is spent hashing out Ids. These are critical to making the messaging and filtering system work at all, but a big win would be making this more efficient. Some possibilities are:
- Use a faster hashing algorithm (https://github.com/servo/rust-fnv)
- Reconstructing the Hasher object over and over again kind of sucks. Making it efficient to concatenate Ids would be a good win here.
- Make Id lazily computed
    - Many primitive elements are given an Id but don't actually use it. Could change the representation of Id so that it stores it's base id + suffix and only computes the real Id when required
- Make string Ids fixed-size (8 characters?)

## Dynamic Allocation
- I've spent a lot of time building my `Arena` buffer for this, but it's not currently used everywhere. Try to replace instances of Box and Stack/Queue with Arena-backed objects where possible.

## Async
- Right now subcontexts have to buffer up all their components before they can do anything. This creates a lot of dynamic allocation (and prevents any sort of threading), would be good to create a system where elements are executed ASAP, using `async`. I have an idea for how this could be done without leaning on the incomplete async system in Rust (that would also work with FFI), using some sort of `ctx.defer(...)` method.
    - Basically a component is ready to run as soon as its created, so put it into some sort of `ready` queue. A deferred element is ready to run as soon as all of its pending elements are ready.

## Threading
- It's a little bit questionable how many threads you actually want to commit to your UI framework, but maybe a good global worker system makes this less of a problem.
- Jobs could be split by `Device`, though that runs the risk of making jobs too small. Some kind of stealing/delegating system would be a good investement here, so threads don't spend more of their time managing jobs than actually doing them.

## Caching
- A robust caching system is a ways off, but I think the "unit" of caching should be `LayoutNode`, since that's easy to re-render, and it's less likely to become invalid than rendering commands (which would be the other candidate). If it's somehow determined that a given `Device` will produce the same `LayoutNode`, then just don't run it.
