use log::*;
use screeps::objects::{Structure, Source, ConstructionSite, SharedCreepProperties, StructureProperties, HasPosition};
use screeps::{prelude::*, game, ObjectId, Position, RoomName};
use screeps::constants::find::{STRUCTURES, MY_CONSTRUCTION_SITES};
use screeps::constants::structure::StructureType;
use crate::creeps::Creep;

pub struct Harvester {
	creep: screeps::objects::Creep
}

impl Creep for Harvester {
	fn new(creep: screeps::objects::Creep) -> Self {
		Harvester { creep: creep }
	}

	fn run(&self) {
		if !self.creep.spawning() {
			match self.creep.memory().string("task").unwrap().unwrap().as_str() {
				"harvesting" => {
					let memory = self.creep.memory();
					let source_id = memory.string("source").unwrap().unwrap();
					let object_id: ObjectId<Source> = source_id.parse().unwrap();
					let source: Source = game::get_object_typed(object_id).unwrap().unwrap();

					let position = memory.dict("position").unwrap();
					if let None = position {
						let container: Vec<Structure> = source.pos().find_in_range(STRUCTURES, 1).into_iter().filter(|s| s.structure_type() == StructureType::Container).collect();
						let construction: Vec<ConstructionSite> = source.pos().find_in_range(MY_CONSTRUCTION_SITES, 1).into_iter().filter(|s| s.structure_type() == StructureType::Container).collect();
						if container.len() > 0 {
							memory.set("position", container[0].pos());
						} else if construction.len() > 0 {
							memory.set("position", construction[0].pos());
						}
					}

					let position = memory.dict("position").unwrap();
					if let Some(pos) = position {
						let x = pos.i32("x").unwrap().unwrap();
						let y = pos.i32("y").unwrap().unwrap();
						let room_name = pos.string("roomName").unwrap().unwrap();
						let target = Position::new(x as u32, y as u32, RoomName::new(&room_name).unwrap());
						if self.creep.pos() == target {
							self.creep.harvest(&source);
						} else {
							self.creep.move_to(&target);
						}
					} else {
						if self.creep.pos().in_range_to(&source, 1) {
							self.creep.harvest(&source);
						} else {
							self.creep.move_to(&source);
						}
					}
				}
				_ => {
					self.creep.memory().set("task", "harvesting".to_owned());
					self.creep.memory().del("target");
				}
			}
		}
	}
}