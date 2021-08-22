use screeps::{HasId, SharedCreepProperties};
use screeps::objects::{Creep, Source, StructureSpawn, SpawnOptions, RoomObjectProperties};
use screeps::memory::MemoryReference;
use screeps::constants::creep::{Part, ReturnCode};
use screeps::constants::find::{SOURCES, MY_CREEPS};

pub struct Spawn {
	pub structure: StructureSpawn
}

impl Spawn {
	fn spawn_creep(&self, name: String, body: Vec<Part>, opts: SpawnOptions) -> ReturnCode {
		self.structure.spawn_creep_with_options(&body, &name, &opts)
	}

	pub fn spawn_harvester(&self, transport_available: bool) -> ReturnCode {
		let body = self.harvester_body(transport_available);
		if body.len() > 0 {
			let creeps: Vec<Creep> = self.structure.room().unwrap().find(MY_CREEPS);
			let sources: Vec<Source> = self.structure.room().unwrap().find(SOURCES);
			let available_sources: Vec<Source> = sources.into_iter().filter(|source| !creeps.iter().any(|creep| creep.memory().string("source").unwrap().unwrap() == source.id().to_string())).collect();
			
			let opts = SpawnOptions::new();
			let mem = MemoryReference::new();
			let name = format!("{}-{}", "harvester", screeps::game::time().to_string());
			mem.set("role", "harvester".to_owned());
			mem.set("task", "harvesting".to_owned());
			mem.set("source", available_sources[0].id().to_string());
			let opts = opts.memory(mem);
			self.spawn_creep(name, body, opts)
		} else {
			ReturnCode::NoBodypart
		}
	}

	pub fn spawn_upgrader(&self, transport_available: bool) -> ReturnCode {
		let body = self.worker_body(transport_available);
		if body.len() > 0 {
			let opts = SpawnOptions::new();
			let mem = MemoryReference::new();
			let name = format!("{}-{}", "upgrader", screeps::game::time().to_string());
			mem.set("role", "upgrader".to_owned());
			mem.set("task", "filling".to_owned());
			let opts = opts.memory(mem);
			self.spawn_creep(name, body, opts)
		} else {
			ReturnCode::NoBodypart
		}
	}

	fn harvester_body(&self, transport_available: bool) -> Vec<Part> {
		let energy_available = self.structure.room().unwrap().energy_available();
		let energy_capacity = if transport_available { self.structure.room().unwrap().energy_capacity_available() } else { self.structure.room().unwrap().energy_available() };
		
		if energy_available >= 300 && energy_capacity < 350 {
	      	vec![Part::Move, Part::Carry, Part::Work, Part::Work]
	    } else if energy_available >= 350 && energy_capacity < 450 {
	      	vec![Part::Move, Part::Move, Part::Carry, Part::Work, Part::Work]
	    } else if energy_available >= 450 && energy_capacity < 550 {
	      	vec![Part::Move, Part::Move, Part::Carry, Part::Work, Part::Work, Part::Work]
	    } else if energy_available >= 550 && energy_capacity < 600 {
	      	vec![Part::Move, Part::Move, Part::Carry, Part::Work, Part::Work, Part::Work, Part::Work]
	    } else if energy_available >= 600 && energy_capacity < 650 {
	      	vec![Part::Move, Part::Carry, Part::Work, Part::Work, Part::Work, Part::Work, Part::Work]
	    } else if energy_available >= 650 && energy_capacity < 700 {
	      	vec![Part::Move, Part::Move, Part::Carry, Part::Work, Part::Work, Part::Work, Part::Work, Part::Work]
	    } else if energy_available >= 700 && energy_capacity < 750 {
	      	vec![Part::Move, Part::Move, Part::Carry, Part::Carry, Part::Work, Part::Work, Part::Work, Part::Work, Part::Work]
	    } else if energy_available >= 750 && energy_capacity < 850 {
	      	vec![Part::Move, Part::Move, Part::Move, Part::Carry, Part::Carry, Part::Work, Part::Work, Part::Work, Part::Work, Part::Work]
	    } else if energy_available >= 850 && energy_capacity < 950 {
	      	vec![Part::Move, Part::Move, Part::Move, Part::Move, Part::Move, Part::Carry, Part::Carry, Part::Work, Part::Work, Part::Work, Part::Work, Part::Work]
	    } else if energy_available >= 950 {
	      	vec![Part::Move, Part::Move, Part::Move, Part::Move, Part::Move, Part::Move, Part::Move, Part::Carry, Part::Carry, Part::Work, Part::Work, Part::Work, Part::Work, Part::Work]
	    } else {
	    	Vec::new()
	    }
	}

	fn worker_body(&self, transport_available: bool) -> Vec<Part> {
	    let energy_available = self.structure.room().unwrap().energy_available();
		let energy_capacity = if transport_available { self.structure.room().unwrap().energy_capacity_available() } else { self.structure.room().unwrap().energy_available() };

		if energy_available >= 300 && energy_capacity < 350 {
			vec![Part::Move, Part::Move, Part::Carry, Part::Carry, Part::Work]
		} else {
			Vec::new()
		}
	}
}