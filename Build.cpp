cc::Build::new()
    .cpp(true) // Switch to C++ library compilation.
    .file("foo.cpp")
    .compile("foo");
