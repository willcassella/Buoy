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