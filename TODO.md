# Lighting
- LoS shadow casting
- Non-dithered setting
- Coloured lighting
- Cone light

# Audio
- Embedded Audio File manager?
- Per sound settings (speed, volume etc)
- *Manual queue management*
- *Positional Audio*

# Sprites
- Rotation
- Introduce flipping in animations (cutting sprites in half in many cases)
- Icons/UI sprites
- Zooming
- Mountpoints

# ECS
- Walls/Floors/Decals
- Efficient Tile System (must support SS culling and efficient lookups)

# Text
- Convert to more efficient system (that doesn't rebuild RenderChar)

# Player Controller
- Fix velocity-based movement to allow for faster than max speed impulses
- Additional states (weapons etc)

# Animation Controller
- Make API more usable

# Camera controller
- Camera smoothing (give camera a target location and have it womp over to it with high accel)
- Ideally a system with a state of which camera entity to use
- Allow for different camera modes (aim at cursor, look-ahead, soft follow, hard follow etc)

# Collisions
- Implement ncollide to allow for better general collisions (and maybe physics xd)

# Debugging
- Create a debug renderer for drawing arbitrary rectangles

# UI
- Implement

# Art
- Round out tileset
- Effects
