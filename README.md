<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD041 -->

## Archival

before luneweb 0.3 this fork was being used for bundling since you can fake require paths using the GlobalsContext struct, but maintaining a Fork is just not very easy to do, so it's decided that LuneWeb is gonna do it's own implementation of require once there is need for bundling again.

## Changes from this fork

### CLI changes

$ Build

the `lune build` command can bundle multiple scripts

```shell
lune build path/to/script1.luau path/to/script2.luau path/to/scrpt3.luau
```

the bundler doesn't bundle `.luaurc` yet so it's not gonna work with custom aliases.

```shell
lune build path/to/*.luau
```

### Internal changes

$ GlobalsContext

this struct allows you to customize the global variables that are inserted by the lune-std crate, for now these customizations are limited to adding your own standard libraries and modulescripts to the require function

<img align="right" width="250" src="assets/logo/tilt_svg.svg" alt="Lune logo" />

<h1 align="center">Lune</h1>

<div align="center">
	<div>
		<a href="https://crates.io/crates/lune">
			<img src="https://img.shields.io/crates/v/lune.svg?label=Version" alt="Current Lune library version" />
		</a>
		<a href="https://github.com/lune-org/lune/actions">
			<img src="https://shields.io/endpoint?url=https://badges.readysetplay.io/workflow/lune-org/lune/ci.yaml" alt="CI status" />
		</a>
		<a href="https://github.com/lune-org/lune/actions">
			<img src="https://shields.io/endpoint?url=https://badges.readysetplay.io/workflow/lune-org/lune/release.yaml" alt="Release status" />
		</a>
		<a href="https://github.com/lune-org/lune/blob/main/LICENSE.txt">
			<img src="https://img.shields.io/github/license/lune-org/lune.svg?label=License&color=informational" alt="Lune license" />
		</a>
	</div>
</div>

<br/>

A standalone [Luau](https://luau-lang.org) runtime.

Write and run programs, similar to runtimes for other languages such as [Node](https://nodejs.org), [Deno](https://deno.land), [Bun](https://bun.sh), or [Luvit](https://luvit.io) for vanilla Lua.

Lune provides fully asynchronous APIs wherever possible, and is built in Rust 🦀 for speed, safety and correctness.

## Features

- 🌙 Strictly minimal but powerful interface that is easy to read and remember, just like Luau itself
- 🧰 Fully featured APIs for the filesystem, networking, stdio, all included in the small (~5mb zipped) executable
- 📚 World-class documentation, on the web _or_ directly in your editor, no network connection necessary
- 🏡 Familiar runtime environment for Roblox developers, with an included 1-to-1 task scheduler port
- ✏️ Optional built-in library for manipulating Roblox place & model files, and their instances

## Non-goals

- Making programs short and terse - proper autocomplete / intellisense make using Lune just as quick, and readability is important
- Running full Roblox games outside of Roblox - there is some compatibility, but Lune is meant for different purposes

## Where do I start?

Head over to the [Installation](https://lune-org.github.io/docs/getting-started/1-installation) page to get started using Lune!
