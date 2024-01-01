# Compatibility to C++ glog

## Missing features

- [ ] Logging to files
  - [x] Writing header
  - [x] Writing log entries
  - [x] Writing backtrace
  - [x] Symlinks
  - [ ] Filesize limits
  - [ ] Filerotation
- [ ] `LOG_IF` macros
- [ ] `VLOG` macros
- [ ] `CHECK` macros
- [ ] Additional log levels
  - [ ] `FATAL` #9
  - [ ] `VERBOSE` #10
- [ ] Flags
  - [ ] Logging in UTC #3
  - [ ] Change some flags during runtime #4
  - [ ] Remove extensions from filename

## glog-rs extensions

- [x] `TRACE` and `DEBUG` levels
- [ ] Filepath instead of filename
- [ ] Crate in addition to filepath/filename
- [ ] Flags
  - [x] Year in log timestamp
