# Chess-rs

This is just a Rust version of my Go chess game. I'm prioritizing multiplayer first and this is being in conjunction to my Go implementation. This is roughly based on the `htmx-refine` branch as it aims to better use backend developement with HTMX as a helper.

## Tools and more

 - [cargo-watch](https://github.com/watchexec/cargo-watch)
 - [Rocket](https://rocket.rs/)
 - [Tera](https://keats.github.io/tera/)
 - [HTMX](https://htmx.org/)
 - [TailwindCSS](https://tailwindcss.com/)
 - [PNPM](https://pnpm.io/)

## Commands and Scripts

 - `pnpm dev`: Allows for continuous development
 - `pnpm build`: Builds the final binaries and assets to build directory
 - `pnpm dev:run`: Runs a single instance of the server
 - `pnpm clean:rel`: Cleans the release directory
 - `pnpm clean:dev`: Cleans the development directory
 - `pnpm clean:all`: Cleans all directories
