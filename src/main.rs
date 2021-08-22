use std::collections::HashSet;

use log::*;
use stdweb::js;
use screeps::{prelude::*};
use screeps::objects::{RoomObjectProperties};
use screeps::constants::find::{SOURCES, MY_CREEPS};

use crate::creeps::Creep;
use crate::creeps::harvester::Harvester;
use crate::creeps::upgrader::Upgrader;

mod logging;
mod spawn;
mod creeps;

fn main() {
    logging::setup_logging(logging::Info);

    js! {
        var game_loop = @{game_loop};

        module.exports.loop = function() {
            // Provide actual error traces.
            try {
                game_loop();
            } catch (error) {
                // console_error function provided by 'screeps-game-api'
                console_error("caught exception:", error);
                if (error.stack) {
                    console_error("stack trace:", error.stack);
                }
                console_error("resetting VM next tick.");
                // reset the VM since we don't know if everything was cleaned up and don't
                // want an inconsistent state.
                module.exports.loop = wasm_initialize;
            }
        }
    }
}

fn game_loop() {
    debug!("loop starting! CPU: {}", screeps::game::cpu::get_used());

    debug!("running spawns");
    for s in screeps::game::spawns::values() {
        debug!("running spawn {}", s.name());
        let spawn = spawn::Spawn { structure: s };
        let sources = spawn.structure.room().unwrap().find(SOURCES);
        let harvesters: Vec<screeps::objects::Creep> = spawn.structure.room().unwrap().find(MY_CREEPS).into_iter().filter(|creep| creep.memory().string("role").unwrap().unwrap() == "harvester").collect();
        let upgraders: Vec<screeps::objects::Creep> = spawn.structure.room().unwrap().find(MY_CREEPS).into_iter().filter(|creep| creep.memory().string("role").unwrap().unwrap() == "upgrader").collect();
        let spawn_transporters: Vec<screeps::objects::Creep> = spawn.structure.room().unwrap().find(MY_CREEPS).into_iter().filter(|creep| creep.memory().string("role").unwrap().unwrap() == "spawn_transporter").collect();
        let transport_available = harvesters.len() > 0 && spawn_transporters.len() > 0;

        if harvesters.len() < sources.len() {
            spawn.spawn_harvester(transport_available);
        } else if upgraders.len() < sources.len() {
            spawn.spawn_upgrader(transport_available);
        }
    }

    debug!("running creeps");
    for creep in screeps::game::creeps::values() {
        match creep.memory().string("role").unwrap().unwrap().as_str() {
            "harvester" => {
                let harvester: Harvester = Creep::new(creep);
                harvester.run();
            }
            "upgrader" => {
                let upgrader: Upgrader = Creep::new(creep);
                upgrader.run();
            }
            _ => { creep.suicide(); }
        }
    }

    /* ---
    debug!("running creeps");
    for creep in screeps::game::creeps::values() {
        let name = creep.name();
        debug!("running creep {}", name);
        if creep.spawning() {
            continue;
        }

        if creep.memory().bool("harvesting") {
            if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
                creep.memory().set("harvesting", false);
            }
        } else {
            if creep.store_used_capacity(None) == 0 {
                creep.memory().set("harvesting", true);
            }
        }

        if creep.memory().bool("harvesting") {
            let source = &creep
                .room()
                .expect("room is not visible to you")
                .find(find::SOURCES)[0];
            if creep.pos().is_near_to(source) {
                let r = creep.harvest(source);
                if r != ReturnCode::Ok {
                    warn!("couldn't harvest: {:?}", r);
                }
            } else {
                creep.move_to(source);
            }
        } else {
            if let Some(c) = creep
                .room()
                .expect("room is not visible to you")
                .controller()
            {
                let r = creep.upgrade_controller(&c);
                if r == ReturnCode::NotInRange {
                    creep.move_to(&c);
                } else if r != ReturnCode::Ok {
                    warn!("couldn't upgrade: {:?}", r);
                }
            } else {
                warn!("creep room has no controller!");
            }
        }
    } --- */

    let time = screeps::game::time();

    if time % 32 == 3 {
        info!("running memory cleanup");
        cleanup_memory().expect("expected Memory.creeps format to be a regular memory object");
    }

    info!("done! cpu: {}", screeps::game::cpu::get_used())
}

fn cleanup_memory() -> Result<(), Box<dyn std::error::Error>> {
    let alive_creeps: HashSet<String> = screeps::game::creeps::keys().into_iter().collect();

    let screeps_memory = match screeps::memory::root().dict("creeps")? {
        Some(v) => v,
        None => {
            debug!("not cleaning game creep memory: no Memory.creeps dict");
            return Ok(());
        }
    };

    for mem_name in screeps_memory.keys() {
        if !alive_creeps.contains(&mem_name) {
            debug!("cleaning up creep memory of dead creep {}", mem_name);
            screeps_memory.del(&mem_name);
        }
    }

    Ok(())
}
