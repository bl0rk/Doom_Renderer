CONTROLS:
- Mouse => Rotate Left/Right and Look Up/Down (except in textured mode).
- T => Toggle texture mode.
- W/S => Walk Forward/Backward.
- A/D => Strafe Left/Right.
- M => Toggle Map.
- Esc => End Game.
- Right Mouse Button => Free mouse from window.

> Controls are handled by the player_input() function in main.rs.

### main.rs
> Main file, sets up the program and handles data transfer and input.

##### main()
- Set up SDL.
- Set up Data.
- Main Game Loop.
- Calls the renderer.

##### player_input()
- Handles all of the input and passes information back into the structs it's given.

### config.rs
> Config file, default settings.

### data.rs
> All Datastructures

### rendering.rs
> The raytracer implementation(s).

##### render_view()
- Currently unused, experiment for buffer based drawing.

##### render_view_canvas()
- Uses rust_sdls canvas to draw to the screen.
- Can toggle between non-textured mode and textured mode.
- Uses DDA to find out where to draw any lines.
