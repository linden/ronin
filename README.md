# Ronin

[![Rust](https://github.com/syvita/ronin/actions/workflows/rust.yml/badge.svg)](https://github.com/syvita/ronin/actions/workflows/rust.yml)

Hello there!

Ronin is a ultra-speed Stacks API server. It's super lightweight, but scales easily.

## Why are we making this?

Because we don't like slow APIs. Also because it's super hard to set up one using current codebases.

![Live footage of us trying to deploy a Stacks API](./repo-img/mendit.gif)

<small align="center"><i>Live footage of us trying to deploy a Stacks API before Ronin</i></small>

## What's the plan?

Simple stuff: Ronin will use the event dispatcher from a Stacks node to hydrate it with that sweet, sweet data.

![Smug face](./repo-img/smug.gif)

- Built in Rust
- Uses Redis as primary datastore (which will fill up all of that juicy RAM of yours)
- Uses the Rocket web framework for *speeeeed*
- Sometimes you won't be able to run it entirely in memory because some people don't have enough RAM to store an entire blockchain in it (what are you doing???). 

Jokes aside, you can set a max limit for the in-memory stuff and we'll use virtual memory that runs on-disk for less-used data (usually old transactions and blocks from the past).

<small align="center"><i>But please, at least run it on an SSD or NVME... I beg you 😭</i></small>

It scales, it descales (is that even a word...) and yeah it's like really fast.

It uses one of those lovely cores on your computer to run the event handler (it handles events from the Stacks node... duh) and uses the rest for the primary API. This means it will run on a Raspberry Pi with an SD card OR you can run it on a 60 core and 128GB RAM monster.

Anyways I'm still working on this so it doesn't work fully yet or at all (lol...).

<small align="center"><i>talk later babez</i></small>
