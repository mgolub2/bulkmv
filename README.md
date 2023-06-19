# bulkmv

A very simple, yet effective rust utility to copy files from one directory to another... in parallel! 

## Motivation

Why? Because Google recently decided to enforce the storage limit on workspace accounts - I had over 12TB stored in one account, which made them very angry, I guess. 

After attempting to use:
 1. Google Takeout, which failed
 2. Data Export via the admin console, which also failed
 3. rclone, which also failed, due to my account having advanced protection enabled
 4. Just zipping the main offending folder in the web interface (arq  backup...) failed...
 5. Google Drive via the Windows Desktop App, which worked, but is single threaded and slow as hell when you have IDK, 200 thousand files to copy...

I decided to write my own tool to do this. It's not pretty, but it works. It opens a bunch of threads (equal to the number of CPUs you have), and copies files from one directory to another. It's not smart, it's not safe, but it's fast.  It's also not very configurable, but it's not meant to be.

Importantly, it builds the list of files in memory first, which is why something like robocopy with /MT did not seem to work for me - the throughput was very poor, possibly due to the fact that it was constantly reading the directory structure on the fly from the disk mounted by the Google Drive App. Or that it is single threaded per folder? Windows is strange, IDK. 

So anyway, if you have a lot of files, this will take some time to build the list of files, but once it does, it will copy them very quickly. If you have A LOT of files, you may run out of memory. By more RAM, you cheapskate!

## Features

 * **Written in Rust. (I love Rust!)**
 * **Parallel copying via rayon. (I love rayon!)**
 * Progress bar via indicatif. (I love indicatif!)
 * Literally zero configuration.
 * Literally zero flags. (Okay, the help and version flags exist, but that's it!)
 * **Literally zero safety checks - it will overwrite files without asking.** 
 * Retries failed operations! (Forever!)
 * **Works on Windows, Linux and MacOS :D**

## Installation

For now, you'll have to build from source. Make sure rust and cargo are installed, then run the following commands:

```
git clone https://github.com/mgolub2/bulkmv.git
cd bulkmv
cargo install --path .
```


## Usage

```
Usage: bulkmv [SRC] [DEST]

Arguments:
  [SRC]   The path to the source directory
  [DEST]  The path to the destination directory

Options:
  -h, --help     Print help
  -V, --version  Print version
```