# Rendering - Part 1

The minifb crate uses a 1D array but to make it easier for us to understand the output window we will add an RGBA pixel buffer which is a 2D array. 
We will also set up an RGBA struct to make it easier to work with the pixel colour values.

To make the program a little more interactive we will let the user cycle through rendering a square in Red, Green and Blue.
We will also make is so when the ESC key is pressed the program exits.
