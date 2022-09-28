# tcpsplit: split a TCP stream into a reader / writer pair
Bart Massey 2022

Written in response to this
[Reddit comment](https://www.reddit.com/r/rust/comments/xqeupd/comment/iq8x9ev/),
this library crate provides a tiny function to split a TCP
stream, yielding a `BufReader` for the read side and a
`BufWriter` for the write side. A demo example `splitdemo`
is also provided.

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
