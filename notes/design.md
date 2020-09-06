Future features:
- Passes: Useful for UIs where an object's layout depends on the layout of one or more "distant" objects
    - Example: A node graph, need to draw curves between where the endpoints end up. Rather than guessing, just output the final position as message and then re-enter the element phase and draw on top.
    - User should still be able to composite with layers from the previous phases, however
    - Too early to decide if input messages should be written between phases or not (probably not)

- Layers: Need an easy way to ensure that different bits of the UI appear on top/behind other parts without a bunch of weird "depth" hacks
    - Same goes for input occlusion/cancelling

- Focus/keyboard navigation:
    - Need some way to make it so that elements are "focusable", and that focused can be moved around in a logical way with shift/shift+tab/arrow keys
    - Not really sure how to expose this at the moment without introducing the concept into every single element. For some elements it may be difficult to implement generally, like grids.
    - A solution would to have it be a parameter for elements that know how to do it, and totally non-existant for elements that don't
        - Kind of raises the question about what you're supposed to do for filters though
    - The focus-flow should be mostly automatic, but the UI designer should be able to change it if they know better
    - Non-trivial case: Changing focus needs to scroll the page
        - Could be solved by making the 'scroll' message higher-level than just based on mouse input

- Some sort of data-driven format. Thinking I'll go with XML with a simple lisp-based scripting language, but that could be a whole other project so I'm deferring that for now.
    - Next steps towards this should be stripping back the "pretty" APIs (and putting them into a wrapper crate)
    - Could use a fancy macro for the time being



# New features
- There's two problems I'm hoping to solve right now with the design.
1) Sockets
    - Right now you can open a socket for an arbitrary number of children. The problem with this is that *you* are responsible for allocating space for those children, and rendering them. It would be nice if there was a way to create
    a subdevice that is re-instantiated for each socket item, so you could use a list + item wrapper
    - I think a potentially simpler and more powerful way of doing this would be to just expose the number of children pushed for a given socket (or even, expose the whole socket:children dictionary abstractly).
        - Could this encourage stalling?
        - How does this work with the "global" sockets idea?
        - This potentially complicates the API, but also allows for more flexibility
            - I don't think caching is a concern here because you could just not call the method to be exempt from that sort of cache invalidation
    - So what does that solve?
        - Now you could do something like 'for x in 0..num_in_socket: push foo_socket_wrapper'
        - Potentially a better solution would be to make the socket API more verbose (in addition?)

    - The only things that matter are top-level calls to ctx.layout(max_size, ....)
        - The ... could be a device, or a socket
        - If it's a device it can have children associated with a socket (except for the root)

Changes:
- LayoutContext will expose a 'socket_children' method, for counting children in a specific socket
- LayoutContext will expose an 'iter_local_sockets' method, to iterate over local sockets and the number of children pushed to them
- SubLayoutContext will be removed in favor of a general-purpose 'LayoutTree' trait
    - More optimal for frontends which build their own tree
- connect_socket() will now take an optional limit parameter

- More features:
- Passes
    - Allow you to do multi-pass layout, which lets the layout of one thing depend on the layout of another, non-local thing.
        - Use case: Drawing curves between nodes in a node graph
- Layers
    - Allows you to composite multiple things and apply shaders. This will most likely be a largely canvas-dependent feature, but notably is orthagonal to passes (multiple passes can reside on the same layer)
- Global sockets
    - Dependent on passes
    - Unanswered questions:
        - When do they close? After the current pass or after the current frame?
            - Thinking after the current pass, since you can re-open them if you want to do another pass
        - Where can you use them?
            - Do they have to be part of an isolated device, or can you put them anywhere, and the scheduling system will just figure it out?
            - Think they have to be isolated devices, otherwise you could create cyclic layout dependencies
- Filters
    - How is precedence handled for overlapping filters?
        - Probably should go to creator of device
    - How do you indicate you want a device to be filtered? Is it based on specific types of "psuedo-devices", or can you mark any device as filterable?
        - Preference towards the former, because it has the simplest caching implications
