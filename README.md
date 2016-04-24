# SBS

_sbs_ is a tool designed to download, install and manage OpenJDKs.

## Benefits for Beginners

Stop worrying about setting up a Java environment, _sbs_ does everything you need!

## Benefits for Experts

Easily manage your OpenJDK installs.

- If a project requires a JDK version you haven't installed yet, _sbs_ does it automatically.
- Insulate yourself from the ever-changing OpenJDK versions of your operating system:
  If you want your project to be compiled or run against a specific version, _sbs_ makes sure you get that version.
- Reproducible builds: _sbs_ ensures that contributors don't have to deal with failures due to different OpenJDK versions.
  They will get exactly the version you want them to use.

## Status

This is an early prototype. It currently downloads and installs a hard-coded OpenJDK version as well as the SBT launcher.
