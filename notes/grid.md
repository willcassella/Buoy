# Grid
The `grid` element is one of the most versatile layout tools, and its really critical that its both easy to use and
performant. I'm mostly taking inspiration from the `grid` element in XAML, I think the grid layout tools available in
the latest version of CSS are *way* too complicated. You can do wacky things like have a grid with two `auto` columns, and
then put long paragraphs of text in them such that both paragraphs wrap correctly so that the grid doesn't overflow. If
you think about the process behind that, it basically requires running layout over each paragraph twice (first to figure
out how big they are, second to figure out how to wrap them when you realize they can't both fit). While that's
convenient I guess, and sort of makes sense for the domain of HTML (where most content is static), I think that's way
too complicated and I don't really see the use since you're giving up a lot of control over your resulting layout there.
For my library I'm leaving it up to the user to figure out how big their containers need to be.

# Design
The grid is specified as a list of rows, columns, and regions. Each of the rows and columns can be auto-sized or
rem-sized (like star-sizing in xaml: each are given a proportion of the remaining space). I may introduce more control
such as min/max directly into the columns, but for now I'm leaving that as the responsibility of the `size` element.
Idea: If no columns or rows are specified, then by default there is a single 'auto' column/row.
Idea: Have additional -1 and +1 auto-columns/rows which start at the edges of the grid. That way you can have elements
that overflow.
