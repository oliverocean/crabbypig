## CrabbyPig

An open source CLI tool for systems administrators.
Generates high entropy, yet easy to type, "[xkcd-style](https://xkcd.com/936/)" passwords.

### Features

* A port of the [OxPig](https://github.com/oliverocean/oxpig) tool, retaining comparable functionality
* Written in the Rust language (the community mascot is a crab)
* Uses the external library ['clap'](https://github.com/clap-rs/clap), a Command Line Argument Parser for Rust
* Options include word count, word length (minimum and maximum), and dictionary source

### Usage

~~~
% crabbypig -h

  CrabbyPig: a Rust language Password Idea Generator
  Source code: <https://github.com/oliverocean/crabbypig>
  Released under the GNU GPL, 2020
  
Usage: crabbypig [FLAGS] [OPTIONS]

Flags:
  -h, --help                 Help information
  -v, --version              Version information

Options:
  -m, --min <minimum>        Minimum word length
  -M, --max <maximum>        Maximum word length
  -w, --wc <word_count>      Total number of words
  -d, --dict <dictionary>    Dictionary source file
~~~

### Examples

~~~
% crabbypig
Press <ENTER> key to continue...
correcthorsebatterystaple
%
~~~

~~~
% crabbypig -m 5 -M 8 --wc 5 --src ./mydict.txt
Press <ENTER> key to continue...
coffeebatterystaplehorsecorrect
%
~~~

### Notes
Development is currently in progress and this tool should be considered beta (for now). Comments in the code indicate areas that are being actively worked on (wip) and/or additional functionality expected in the next version. (This readme will be updated when minimum viable product is available!)
