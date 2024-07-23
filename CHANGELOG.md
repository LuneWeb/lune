<!-- markdownlint-disable MD023 -->
<!-- markdownlint-disable MD033 -->

# Changelog

All notable changes to this project will be documented in this file.

## `0.1.2`

### Fixes

- The build command now resolves relative paths which should solve a few issues
  with binaries not working in different directories

## `0.1.1`

### Fixes

- Binaries built with the build command should no longer error when theyre too small
  or too big

## `0.1.0`

### Changes

- The build command now compresses the binary using the LZ4 algorithm
