# Duplicate Explorer

## introduction
This cli program allows you to rename filename in bulk, it also deletes file with specific patterns, find duplicate files and it also works as a file explorer.

## motivation
So i download a lot of videos from youtube because you can only watch one video at a time obviously. and sometimes i download videos from youtube. the video name starts with `y2mate.com - a - ` which i don't like.

## quick start
1. clone the repo
`https://github.com/ahmad-olu/Duplicate-Explorer.git`
2. run the command below replacing each parts with the using the usage and make sure your feature flag is replace to use the replace feature
`cargo run -- -f replace -p C:\Users\pc\Downloads\test -c aCopy -r bCopy`
## usage

```
    -f, --feature <FEATURE>            the type of feature
    -p, --path <PATH>                  the path to read
    -c, --contains <CONTAINS>          the value to be replaces
    -r, --replace-with <REPLACE_WITH>  What to replace it with
    -h, --help                         Print help
    -V, --version                      Print version
```
## contribution
--------------
# This project is not completed yet 

### other feature to come
- duplicate file finder(both with hashing and just file name)
- deleting file with a particular pattern
- file explorer
--------------