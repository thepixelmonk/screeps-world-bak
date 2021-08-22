use log::*;
use screeps::objects::{Structure, Source, Resource, ConstructionSite, SharedCreepProperties, StructureProperties, HasPosition};
use screeps::{prelude::*, game, ObjectId, Position, RoomName};
use screeps::constants::ResourceType;
use screeps::constants::find::{STRUCTURES, MY_CONSTRUCTION_SITES, DROPPED_RESOURCES};
use screeps::constants::structure::StructureType;
use crate::creeps::Creep;

pub struct Upgrader {
	creep: screeps::objects::Creep
}

impl Creep for Upgrader {
	fn new(creep: screeps::objects::Creep) -> Self {
		Upgrader { creep: creep }
	}

	fn run(&self) {
		if !self.creep.spawning() {
			match self.creep.memory().string("task").unwrap().unwrap().as_str() {
				"filling" => {
					let memory = self.creep.memory();

					let target = memory.string("target").unwrap();
					if let None = target {
						let mut resources: Vec<Structure> = self.creep.room().unwrap().find(STRUCTURES).into_iter().filter(|s| s.structure_type() == StructureType::Container || s.structure_type() == StructureType::Storage).filter(|s| s.as_has_store().unwrap().energy() > 0).collect();
						resources.sort_by(|a,b| b.as_has_store().unwrap().energy().partial_cmp(&a.as_has_store().unwrap().energy()).unwrap());
						let mut dropped: Vec<Resource> = self.creep.room().unwrap().find(DROPPED_RESOURCES);
						dropped.sort_by(|a,b| b.amount().partial_cmp(&a.amount()).unwrap());
						if resources.len() > 0 {
							memory.set("target", resources[0].id().to_string());
							memory.set("type", "structure".to_owned());
						} else if dropped.len() > 0 {
							memory.set("target", dropped[0].id().to_string());
							memory.set("type", "resource".to_owned());
						}
					}

					let target = memory.string("target").unwrap();
					if let Some(target_id) = target {
						let target_type = memory.string("type").unwrap().unwrap();
						if target_type == "structure" {
							let object_id: ObjectId<Structure> = target_id.parse().unwrap();
							let option = game::get_object_typed(object_id).unwrap();
							if let Some(container) = option {
								if self.creep.pos().in_range_to(&container, 1) {
									self.creep.withdraw_all(container.as_withdrawable().unwrap(), ResourceType::Energy);
									if self.creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
										memory.set("task", "upgrading".to_owned());
										memory.del("target");
										memory.del("type");
									}
								} else {
									self.creep.move_to(&container);
								}
							} else {
								memory.del("target");
								memory.del("type");
							}
						} else {
							let object_id: ObjectId<Resource> = target_id.parse().unwrap();
							let option = game::get_object_typed(object_id).unwrap();
							if let Some(resource) = option {
								if self.creep.pos().in_range_to(&resource, 1) {
									self.creep.pickup(&resource);
									if self.creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
										memory.set("task", "upgrading".to_owned());
										memory.del("target");
										memory.del("type");
									}
								} else {
									self.creep.move_to(&resource);
								}
							} else {
								memory.del("target");
								memory.del("type");
							}
						}
					}
				}
				"upgrading" => {
					let controller = self.creep.room().unwrap().controller().unwrap();
					if self.creep.pos().in_range_to(&controller, 3) {
						self.creep.upgrade_controller(&controller);
					} else {
						self.creep.move_to(&controller);
					}
				}
				_ => {
					self.creep.memory().set("task", "filling".to_owned());
					self.creep.memory().del("target");
				}
			}
		}
	}
}