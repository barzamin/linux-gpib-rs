# linux-gpib-sys
## Backstory
GPIB is a digital communications protocol, also known as IEEE 488. It was initially developed as HP-IB for HP's test equipment, to provide a channel for management and automated control, and is still relatively widely used for that purpose.

The most popular way to speak GPIB is a PCI(-E) card or USB adaptor, often using NI's GPIB library on top of their VISA hardware management tools. However, VISA _sucks_ and doesn't run on Linux. However! Some very nice people wrote drivers for a bunch of popular GPIB controllers and packaged them up into [linux-gpib](https://linux-gpib.sourceforge.io/), a collection of kernel driver modules plus a C user-space library to interact with them. The user-space API is basically one-to-one with NI's GPIB APIs, so the massive amounts of documentation on that carry over.

This is just a Rust binding to the linux-gpib userspace API. At the moment, it's just a `-sys` crate; at some point in the future, I'll probably write a nice layer over it. Examples are forthcoming, too!

## License
```
BSD 3-Clause License

Copyright (c) 2019, Erin Moon <erin@hecke.rs>
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```