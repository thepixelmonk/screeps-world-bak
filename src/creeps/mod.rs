pub mod harvester;
pub mod upgrader;

pub trait Creep {
	fn new(creep: screeps::objects::Creep) -> Self;
	fn run(&self) -> ();
}