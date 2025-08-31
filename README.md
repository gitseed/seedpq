# seedpq
libpq based rust postgres library.

## Differences between seedpq and other postgres libraries

* seedpq uses channels for parallelism, which I don't think other libraries do

## Disadvantages of seedpq compared to other rust postgres libraries

* seedpq is maintained by myself, Paul Dejean, who currently works at Civitas Learning, and who maintains this project in their individual capacity in their free time. Other rust postgres libraries generally have more maintainers who have more time to devote to their projects.
* seedpq uses unsafe rust, because it utilizes a C library. This means there's no way to programatically guarantee the safety of the code. Of course I will aim to make the code safe, but I'm human so it's possible for me to make mistakes.
* seedpq is primarily written to help me write gitseed, and its patterns might be inconvenient for someone who has different postgres usage patterns.
* seedpq is not widely used, and isn't used in production anywhere I know of. Other postgres libraries are likely to be far more "battle tested."

## Build requirements

seedpq has the following build dependencies that aren't well expressed in Cargo.toml

* libpq version 17+, the C library. seedpq is built on top of libpq so libpq is required to be installed. We require version 17 or greater as we use the  PQsetChunkedRowsMode which was introduced in version 17.
* openssl, the C library. libpq uses openssl for cryptography, so openssl is required to be intalled. It must be a openssl version that is compatible with your libpq version.
* pkg-config, the build tool. pkg-config the cli program is used by the crate pkg-config to determine where C libraries are located and pass that to cargo. If you don't have pkg-config installed or you don't want to use pkg-config for some reason, then you'll have to hack the build.rs file.
