# rustylantern (working title)

A standalone 2d game engine (StonEng) and a currently untitled game.
The goal of the project is to fully implement a reusable game engine within StoneEng library
as well as develop an open-source game for eventual release.

As it stands, StoneEng functions as the game's renderer, driver and ECS backend (through 'Specs').

### Major dependencies
- [Specs](https://github.com/amethyst/specs) for the basis of the Entity-Component-System.
- [glutin](https://github.com/rust-windowing/glutin) for windowing and OpenGL context.
- [gl](https://github.com/brendanzab/gl-rs) for OpenGL bindings.
- [nalgebra-glm](https://github.com/dimforge/nalgebra) for linear algebra.
- [stb-image](https://github.com/servo/rust-stb-image) for image loading.

### Author
Jayden Dumouchel -- jdumouch@ualberta.ca | rottenfishbone@pm.me
