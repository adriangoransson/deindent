# deindent

A command line utility and Rust library to format overly-indented text.

<img width="716" alt="" src="https://github.com/user-attachments/assets/2da60fac-d3c7-4999-8f6c-866bd5c45bbb" />

## Installation

At the moment, a Rust (or more specifically Cargo) installation is needed. To
install, run:

```sh
$ cargo install deindent
```

## Example uses

My primary use-case for this utility is to deindent text that I'm copying from
an editor to e.g. my browser.

### Deindent system clipboard

Set up an alias in your shell that deindents your clipboard content.

- macOS:

```sh
$ alias pbdeindent='pbpaste | deindent | pbcopy'
```

- Linux (Wayland with [`wl-clipboard`](https://github.com/bugaevc/wl-clipboard))

```sh
$ alias wl-deindent='wl-paste | deindent | wl-copy'
```

### (Neo)Vim

The following Vimscript configures (Neo)Vim to automatically deindent the `*`
(clipboard) register after yanking to it.

```vim
if executable("deindent")
    augroup DeindentClipboardRegister
        autocmd!
        autocmd TextYankPost *
            \ if v:event["regname"] == "*" |
            \ call setreg("*", system("deindent", getreg("*"))) |
            \ endif
    augroup END
endif
```
