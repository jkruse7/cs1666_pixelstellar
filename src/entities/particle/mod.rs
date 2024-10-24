// lets try to follow this module scheme for entities:

// >src
//  >entity_1
//   >mod.rs
//   >components.rs
//   >resources.rs
//   >systems.rs
//  >entity_2
//   >...
//  >main.rs

pub mod components;
pub mod resources;
pub mod systems;