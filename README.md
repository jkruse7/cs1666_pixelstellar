# PixelStellar

by planeteers

## Team Members
* Advanced Topic Subteam 1: Physics Simulation
   * Kory Sanchez
   * Bairun Chen
   * Julianne Kruse
   * Jordan Shopp

* Advanced Topic Subteam 2: Procedural Generation  
  * Elijah Morgan
  * Zixin Ye
  * Adam Gimbel
  

## Game Description

Pixelstellar will be a sandbox-esque game with a simulated world, similar to the game Noita. There will be multiple elements in the world which can be manipulated and all have different properties. The gameplay will involve exploring procedurally generated worlds with different biomes. The player will act as an astronaut exploring new planets/worlds they encounter in deep space. There will be 7 planets (one per team member) that the player will travel to.
Each team member will come up with their own planet (artwork, idea, and at least one unique attribute or property of the enemy that spawns there). Progress through each planet will come from killing all the available enemies on that planet. The goal of the player will be to successfully explore/survive the 7 planets (by successfully killing the enemies they encounter).


## Advanced Topic Description

### Physics Simulation

The base of the game will revolve around a simulated 2d world, where all active (foreground) pixels will have a particular element with its own set of unique properties. For instance, liquids should flow with gravity, while gasses will float into the air. Potential later elements may include fire, air (wind), and more. The advanced topic here will involve simulating an entire screen of interactable pixels while maintaining performance, as well as allowing all of the elements to behave in a realistic manner.
    
### Procedural Generation

Each planet will be procedurally generated with terrain, natural structures (mountains, caves, etc.) and possibly manmade structures (mineshafts, bastions, etc.). There will be a different biome for each planet with biome specific generation. The advanced topic here will involve many algorithms which generate randomness such as perlin noise. These algorithms will be fine tuned and layered for each planet to generate “realistic” looking terrains.

## Midterm Goals

* A walkable player character, possibly with a functional weapon.
* Scrollable world
* Basic enemy prototype
* A good framework for creating elements/materials (physics implementation, etc.).
* Physics: a few base elements with basic functionality (i.e. one liquid and one solid).
* Procedural generation: some type of noise generator that is scalable for different types of environments as well as a pack spawning algo for enemies.

## Final Goals

* 10%: Basic player movement
* 10%: Scrollable simulated world
* 10%: Prototype enemies
* 20%: (Procedural) Procedural generation of at least 2 different generators
* 20%: (Physics) Working physics elements of water, gas, and solid types.
* 10%: 7 Worlds of size ( 5 * (1280x720) )
  * Each team member should create a planet and come up with:
	  * Name of the planet
	  * Artwork for enemies there
	  * Unique enemy with one special property
	  * Unique element with one special property
	  * The type of environment and generation of the world

## Stretch Goals

* Implementing rain in game. Rain will react naturally to environment (ex. rain will erode dirt)
* GOAL2HERE
