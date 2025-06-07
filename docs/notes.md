# Goal

- Try to implement a embedable KV store with very little dependencies
- 100% will need to use stuff like libc but I'd like to try to do it in Rust and limit `unsafe` usage

## Why?

- get a super deep technical dive into what makes up the lowest level DBs like rocksdb, lmdb, leveldb
- most modern new database startups / technical teams usually wrap around one of the 3 big embedable KV stores ^

## still trying to map out

- using LSM vs Btrees (copy on write or another derivative) but looking to use LSM

## other notable projects

- https://github.com/cberner/redb
- has aprox 25,000 LOC, easy to read and understand
- uses Btree, so would be cool to do LSM

Last Updated June 7th 2025
