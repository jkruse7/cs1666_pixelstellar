# PixelStellar

by Pixel Stellars

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
Each team member will come up with their own planet (artwork, idea, and at least one unique attribute or property of the enemy that spawns there). Progress through each planet will come from killing 5 enemies on that planet, and safely returning to your ship. The goal of the player will be to successfully explore/survive the 7 planets (by successfully killing the enemies they encounter).

Concept Art:  
![image](https://github.com/user-attachments/assets/dae5540b-d718-4aca-8822-08846bba24b8)  
![image](https://github.com/user-attachments/assets/7390b9b0-ca0a-47c6-906f-c3bd39a07241)![image](https://github.com/user-attachments/assets/0957baec-028a-4c0e-ba9d-f98304f5cf5e)




## Advanced Topic Description

### Physics Simulation

The base of the game will revolve around a simulated 2d world, where all active (foreground) pixels will have a particular element with its own set of unique properties. For instance, liquids should flow with gravity, while gasses will float into the air. Potential later elements may include fire, air (wind), and more. The advanced topic here will involve simulating an entire screen of interactable pixels while maintaining performance, as well as allowing all of the elements to behave in a realistic manner.
    
### Procedural Generation

Each planet will be procedurally generated with terrain, natural structures (mountains, caves, etc.). There will be a different biome for each planet with biome-specific generation. The advanced topic here will involve many algorithms that generate randomness, such as Poisson disk sampling. These algorithms will be fine-tuned and layered for each planet to generate "realistic" looking terrains. The end goal is to have infinitely generating terrain in the left and right directions. Additionally, we will integrate two key algorithms to further enhance the complexity and realism of the environment. Cellular automata will be used to aid in real-time particle physics simulation and interactions between different elements, while also generating small environmental variations within the terrain to add dynamic local features. Poisson disk sampling, Perlin noise along with other noise generation algorithms, will be used to create larger, more natural terrain variations, ensuring smoother transitions and realistic landscape formations across the planetary environments.

## Midterm Goals

* A walkable player character, who is able to walk to the left, right, and jump, and will fall with gravity.
* Player will have a weapon that can shoot single particles (pixels) of a single element type
	* This weapon will deal damage to the enemy when particle touches enemy
* Scrollable world, in L to R dimensions
* Basic enemy prototype
	* Enemy should deal damage to the player's HP when it touches the player
 	* Enemy should track towards the player when they are on screen
  	* Enemy should lose HP when hit with particle gun pixels
* A base implementation of an element that can be used to create further elements/materials (physics implementation, etc.).
	* Should have at a minimum two traits that can be implemented, such as falls_with_gravity, is_liquid. 
* Physics: one base element with basic functionality (one liquid (water) that will flow over solid ground and player will sink in liquid)
* Procedural generation: some type of noise generator that is scalable for different types of environments.

## Final Goals

* 10%: Basic player movement
	* Player can jump and walk left and right
 	* Player will have a particle gun that shoots single pixels. Particles will be a unique set element for each world, that has a special attribute (ex. ice particle slows enemy, or acid particle deals damage over time)
  	* Player will have an HP of 100. If HP is fully depleted, player "dies" and must start the level over
* 15%: Prototype enemies
	* 7 unique enemies (one for each world)
 	* Enemies will each have a unique HP that can be depleted when hit by particles from the player's particle gun
  	* When enemies come into contact with the player they will deplete the player's HP by a unique number
  	* Beyond losing HP, if a particle has a special property, enemies will react to the particle they are hit with (ex. ice particle slows enemy)
* 20%: (Procedural) Procedural generation of at least 2 different generators with chunk loading in an infinite direction left and right
* 20%: (Physics) Working physics elements of water, gas, and solid types.
	* Liquids will flow across solids and player will sink in liquids
 	* Gas will float into the air
  	* Player can walk over solids and solids can be destroyed/shot with particle gun
* 15%: 7 Worlds
  * Each team member should create a planet and come up with:
	  * Unique enemy with one special property
	  * Unique element with one special property (this will be used in particle gun)

## Stretch Goals

* Implementing rain in game. Rain will react naturally to environment (ex. rain will erode dirt)
* Rigid body physics, at least implementing a box which is made of pixel elements, but behaves as one object.
