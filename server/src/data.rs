use std::collections::BTreeMap;
pub fn create_recipes() -> Vec<Recipe> {
    vec![
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::MvMotor, 2);
                components.insert(Part::RubberSheet, 6);
                components.insert(Part::CopperCable, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::MvBelt, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Mv Belt",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::MvPiston, 1);
                components.insert(Part::MvMotor, 2);
                components.insert(Part::CopperCable, 3);
                components.insert(Part::AluminiumRod, 2);
                components.insert(Part::MvCircuit, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::MvRobotArm, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Mv Robot Arm",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::AluminiumPlate, 3);
                components.insert(Part::AluminiumRod, 2);
                components.insert(Part::CopperCable, 2);
                components.insert(Part::AluminiumGearSmall, 1);
                components.insert(Part::MvMotor, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::MvPiston, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Mv Piston",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::BronzeScrew, 1);
                components.insert(Part::BronzeRotor, 1);
                components.insert(Part::RubberRing, 2);
                components.insert(Part::SteelPipe, 1);
                components.insert(Part::MvMotor, 1);
                components.insert(Part::CopperCable, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::MvPump, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Mv Pump",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::AluminiumRod, 2);
                components.insert(Part::CopperCable, 2);
                components.insert(Part::CupronickelWirex2, 4);
                components.insert(Part::SteelRodMagnetic, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::MvMotor, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Mv Motor",
        },
        Recipe {
            machine: Machine::CircuitAssembler,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CircuitBoardGood, 1);
                components.insert(Part::LvCircuit, 2);
                components.insert(Part::CopperWire, 2);
                components.insert(Part::Diode, 2);
                components.insert(Part::SolderingAlloyMolten, 72);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::MvCircuit, 1);
                results
            },
            time_hundredths: 1500,
            name: "Assemble Mv Circuit",
        },
        Recipe {
            machine: Machine::Extractor,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::SolderingAlloyDust, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::SolderingAlloyMolten, 144);
                results
            },
            time_hundredths: 725,
            name: "Extract Molten Soldering Alloy",
        },
        Recipe {
            machine: Machine::Mixer,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::TinDust, 6);
                components.insert(Part::LeadDust, 3);
                components.insert(Part::AntimonyDust, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::SolderingAlloyDust, 10);
                results
            },
            time_hundredths: 1000,
            name: "Mix Soldering Alloy Dust",
        },
        Recipe {
            machine: Machine::Extruder,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::AluminiumIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::AluminiumGearSmall, 2);
                results
            },
            time_hundredths: 130,
            name: "Extrude Aluminium Gear Small",
        },
        Recipe {
            machine: Machine::Assembler,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CopperAnnealedWireFine, 4);
                components.insert(Part::GalliumArsenideDustSmall, 1);
                components.insert(Part::GlassMolten, 144);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::Diode, 2);
                results
            },
            time_hundredths: 2000,
            name: "Assemble Diode",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::RubberSheet, 6);
                components.insert(Part::TinCable, 1);
                components.insert(Part::LvMotor, 2);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::LvBelt, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Lv Belt",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::TinCable, 3);
                components.insert(Part::SteelRod, 2);
                components.insert(Part::LvMotor, 2);
                components.insert(Part::LvPiston, 1);
                components.insert(Part::LvCircuit, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::LvRobotArm, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Lv Robot Arm",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::IronWroughtPlate, 3);
                components.insert(Part::IronWroughtRod, 2);
                components.insert(Part::TinCable, 2);
                components.insert(Part::IronWroughtGear, 1);
                components.insert(Part::LvMotor, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::LvPiston, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Lv Piston",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::TinScrew, 1);
                components.insert(Part::TinRotor, 1);
                components.insert(Part::RubberRing, 2);
                components.insert(Part::BronzePipe, 1);
                components.insert(Part::LvMotor, 1);
                components.insert(Part::TinCable, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::LvPump, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Lv Pump",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::IronRod, 2);
                components.insert(Part::TinCable, 2);
                components.insert(Part::CopperWireFine, 4);
                components.insert(Part::IronRodMagnetic, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::LvMotor, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Lv Motor",
        },
        Recipe {
            machine: Machine::Extruder,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::BronzeIngot, 4);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::BronzeRotor, 4);
                results
            },
            time_hundredths: 1520,
            name: "Extrude Bronze Rotor",
        },
        Recipe {
            machine: Machine::Extruder,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::SteelIngot, 3);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::SteelPipe, 1);
                results
            },
            time_hundredths: 840,
            name: "Extrude Steel Pipe",
        },
        Recipe {
            machine: Machine::Wiremill,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CopperIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CopperWire, 2);
                results
            },
            time_hundredths: 315,
            name: "Mill Copper Wire",
        },
        Recipe {
            machine: Machine::Lathe,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::IronWroughtIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::IronWroughtRod, 2);
                results
            },
            time_hundredths: 560,
            name: "Lathe Wrought Iron Rod",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::TinWire, 1);
                components.insert(Part::RubberSheet, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::TinCable, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Tin Cable",
        },
        Recipe {
            machine: Machine::Extruder,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::IronWroughtIngot, 4);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::IronWroughtGear, 4);
                results
            },
            time_hundredths: 1400,
            name: "Extrude Wrought Iron Gear",
        },
        Recipe {
            machine: Machine::Extruder,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::RubberIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::RubberRing, 4);
                results
            },
            time_hundredths: 50,
            name: "Extrude Rubber Ring",
        },
        Recipe {
            machine: Machine::CircuitAssembler,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CircuitBoard, 1);
                components.insert(Part::Resistor, 2);
                components.insert(Part::RedAlloyWire, 2);
                components.insert(Part::VacuumTube, 2);
                components.insert(Part::SolderingAlloyMolten, 72);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::LvCircuit, 2);
                results
            },
            time_hundredths: 1000,
            name: "Assemble Lv Circuit",
        },
        Recipe {
            machine: Machine::Lathe,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::IronIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::IronRod, 2);
                results
            },
            time_hundredths: 560,
            name: "Lathe Iron Rod",
        },
        Recipe {
            machine: Machine::BendingMachine,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::IronWroughtIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::IronWroughtPlate, 2);
                results
            },
            time_hundredths: 280,
            name: "Bend Wrought Iron Plate",
        },
        Recipe {
            machine: Machine::Wiremill,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CupronickelIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CupronickelWirex2, 1);
                results
            },
            time_hundredths: 600,
            name: "Mill Cupronickel Wirex2",
        },
        Recipe {
            machine: Machine::Lathe,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::BronzeBolt, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::BronzeScrew, 1);
                results
            },
            time_hundredths: 45,
            name: "Lathe Bronze Screw",
        },
        Recipe {
            machine: Machine::Lathe,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::TinBolt, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::TinScrew, 1);
                results
            },
            time_hundredths: 70,
            name: "Lathe Tin Screw",
        },
        Recipe {
            machine: Machine::ElectromagneticPolarizer,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::SteelRod, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::SteelRodMagnetic, 1);
                results
            },
            time_hundredths: 140,
            name: "Polarize Steel Rod",
        },
        Recipe {
            machine: Machine::Assembler,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::GlassTube, 1);
                components.insert(Part::SteelBolt, 1);
                components.insert(Part::CopperAnnealedWire, 2);
                components.insert(Part::RedAlloyMolten, 18);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::VacuumTube, 4);
                results
            },
            time_hundredths: 200,
            name: "Assemble Vacuum Tube",
        },
        Recipe {
            machine: Machine::Assembler,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CarbonDust, 1);
                components.insert(Part::CopperAnnealedWireFine, 4);
                components.insert(Part::Glue, 100);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::Resistor, 4);
                results
            },
            time_hundredths: 800,
            name: "Assemble Resistor",
        },
        Recipe {
            machine: Machine::ElectromagneticPolarizer,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::IronRod, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::IronRodMagnetic, 1);
                results
            },
            time_hundredths: 140,
            name: "Polarize Iron Rod",
        },
        Recipe {
            machine: Machine::Extruder,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::SteelIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::SteelBolt, 8);
                results
            },
            time_hundredths: 75,
            name: "Extrude Steel Bolt",
        },
        Recipe {
            machine: Machine::Lathe,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::AluminiumIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::AluminiumRod, 2);
                results
            },
            time_hundredths: 260,
            name: "Lathe Aluminium Rod",
        },
        Recipe {
            machine: Machine::Extractor,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::Glass, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::GlassMolten, 144);
                results
            },
            time_hundredths: 100,
            name: "Extract Molten Glass",
        },
        Recipe {
            machine: Machine::Extruder,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::TinIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::TinBolt, 8);
                results
            },
            time_hundredths: 75,
            name: "Extrude Tin Bolt",
        },
        Recipe {
            machine: Machine::Furnace,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::IronIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::IronWroughtIngot, 1);
                results
            },
            time_hundredths: 63,
            name: "Smelt Wrought Iron Ingot",
        },
        Recipe {
            machine: Machine::AlloySmelter,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CopperIngot, 1);
                components.insert(Part::NickelIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CupronickelIngot, 2);
                results
            },
            time_hundredths: 500,
            name: "Alloy Smelt Cupronickel Ingot",
        },
        Recipe {
            machine: Machine::Wiremill,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CopperAnnealedIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CopperAnnealedWireFine, 8);
                results
            },
            time_hundredths: 945,
            name: "Mill Fine Annealed Copper Wire",
        },
        Recipe {
            machine: Machine::Extruder,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::TinIngot, 4);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::TinRotor, 1);
                results
            },
            time_hundredths: 2360,
            name: "Extrude Tin Rotor",
        },
        Recipe {
            machine: Machine::ChemicalReactor,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::Sulfur, 6);
                components.insert(Part::Salt, 12);
                components.insert(Part::IronDust, 2);
                components.insert(Part::Oxygen, 18000);
                components.insert(Part::Water, 6000);
                components.insert(Part::SilverFoil, 140);
                components.insert(Part::PhenolicSubstrate, 35);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CircuitBoardGood, 35);
                results.insert(Part::Hydrogen, 18000);
                results
            },
            time_hundredths: 75400,
            name: "React Circuit Board Good",
        },
        Recipe {
            machine: Machine::BendingMachine,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::AluminiumIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::AluminiumPlate, 1);
                results
            },
            time_hundredths: 130,
            name: "Bend Aluminium Plate",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::GalliumArsenideDust, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::GalliumArsenideDustSmall, 4);
                results
            },
            time_hundredths: 0,
            name: "Craft Small Gallium Arsenide Dust",
        },
        Recipe {
            machine: Machine::Assembler,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CopperFoil, 4);
                components.insert(Part::WoodPlank, 1);
                components.insert(Part::Glue, 100);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CircuitBoard, 1);
                results
            },
            time_hundredths: 1000,
            name: "Assemble Circuit Board",
        },
        Recipe {
            machine: Machine::Extractor,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::RubberSheet, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::RubberMolten, 144);
                results
            },
            time_hundredths: 25,
            name: "Extract Molten Rubber",
        },
        Recipe {
            machine: Machine::FluidSolidifier,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::RubberMolten, 144);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::RubberIngot, 1);
                results
            },
            time_hundredths: 100,
            name: "Solidify Rubber Ingot",
        },
        Recipe {
            machine: Machine::Extruder,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::BronzeIngot, 3);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::BronzePipe, 1);
                results
            },
            time_hundredths: 1140,
            name: "Extrude Bronze Pipe",
        },
        Recipe {
            machine: Machine::Assembler,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CopperWire, 1);
                components.insert(Part::RubberMolten, 144);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CopperCable, 1);
                results
            },
            time_hundredths: 500,
            name: "Assemble Copper Cable",
        },
        Recipe {
            machine: Machine::Wiremill,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CopperAnnealedIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CopperAnnealedWire, 2);
                results
            },
            time_hundredths: 315,
            name: "Mill Annealed Copper Wire",
        },
        Recipe {
            machine: Machine::Crusher,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::LeadIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::LeadDust, 1);
                results
            },
            time_hundredths: 194,
            name: "Crush Lead Ingot",
        },
        Recipe {
            machine: Machine::Extruder,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::BronzeIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::BronzeBolt, 8);
                results
            },
            time_hundredths: 75,
            name: "Extrude Bronze Bolt",
        },
        Recipe {
            machine: Machine::Electrolyzer,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::Water, 1000);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::Hydrogen, 2000);
                results.insert(Part::Oxygen, 1000);
                results
            },
            time_hundredths: 7500,
            name: "Electrolyze Water",
        },
        Recipe {
            machine: Machine::Wiremill,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CopperIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CopperWireFine, 8);
                results
            },
            time_hundredths: 945,
            name: "Mill Fine Copper Wire",
        },
        Recipe {
            machine: Machine::Lathe,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::SteelIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::SteelRod, 2);
                results
            },
            time_hundredths: 560,
            name: "Lathe Steel Rod",
        },
        Recipe {
            machine: Machine::Centrifuge,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::StickyResin, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::Glue, 100);
                results.insert(Part::RawRubberPulp, 3);
                results
            },
            time_hundredths: 2000,
            name: "Centrifuge Sticky Resin",
        },
        Recipe {
            machine: Machine::BendingMachine,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::SilverPlate, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::SilverFoil, 4);
                results
            },
            time_hundredths: 535,
            name: "Bend Silver Foil",
        },
        Recipe {
            machine: Machine::BendingMachine,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::SilverIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::SilverPlate, 1);
                results
            },
            time_hundredths: 535,
            name: "Bend Silver Plate",
        },
        Recipe {
            machine: Machine::BendingMachine,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CopperPlate, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CopperFoil, 4);
                results
            },
            time_hundredths: 315,
            name: "Bend Copper Foil",
        },
        Recipe {
            machine: Machine::BendingMachine,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CopperIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CopperPlate, 1);
                results
            },
            time_hundredths: 315,
            name: "Bend Copper Plate",
        },
        Recipe {
            machine: Machine::Extractor,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::RedAlloyIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::RedAlloyMolten, 144);
                results
            },
            time_hundredths: 400,
            name: "Extract Molten Red Alloy",
        },
        Recipe {
            machine: Machine::AlloySmelter,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CopperIngot, 1);
                components.insert(Part::RedStoneDust, 4);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::RedAlloyIngot, 1);
                results
            },
            time_hundredths: 250,
            name: "Alloy Smelt Red Alloy Ingot",
        },
        Recipe {
            machine: Machine::Wiremill,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::RedAlloyIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::RedAlloyWire, 2);
                results
            },
            time_hundredths: 400,
            name: "Mill Red Alloy Wire",
        },
        Recipe {
            machine: Machine::Crusher,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::TinIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::TinDust, 1);
                results
            },
            time_hundredths: (590.0 * 1.5 / 8.0) as u32,
            name: "Crush Tin Ingot",
        },
        Recipe {
            machine: Machine::ArcFurnace,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CopperIngot, 1);
                components.insert(Part::Oxygen, 63);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CopperAnnealedIngot, 1);
                results
            },
            time_hundredths: 315,
            name: "Anneal Copper Ingot",
        },
        Recipe {
            machine: Machine::FluidSolidifier,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::GlassMolten, 144);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::GlassTube, 1);
                results
            },
            time_hundredths: 1000,
            name: "Solidify Glass Tube",
        },
        Recipe {
            machine: Machine::Wiremill,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::TinIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::TinWire, 2);
                results
            },
            time_hundredths: 590,
            name: "Mill Tin Wire",
        },
        Recipe {
            machine: Machine::AlloySmelter,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CopperIngot, 3);
                components.insert(Part::TinIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::BronzeIngot, 4);
                results
            },
            time_hundredths: 1000,
            name: "Alloy Smelt Bronze Ingot",
        },
        Recipe {
            machine: Machine::Compressor,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::WoodPulp, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::WoodPlank, 1);
                results
            },
            time_hundredths: 1000,
            name: "Compress Wood Pulp",
        },
        Recipe {
            machine: Machine::Crusher,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::GraphiteIngot, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::GraphiteDust, 1);
                results
            },
            time_hundredths: (60.0 * 1.5 / 8.0) as u32,
            name: "Crush Graphite Ingot",
        },
        Recipe {
            machine: Machine::Electrolyzer,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::GraphiteDust, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CarbonDust, 4);
                results
            },
            time_hundredths: 500,
            name: "Electrolyze Graphite Dust",
        },
        Recipe {
            machine: Machine::Furnace,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::SlimeBall, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::StickyResin, 1);
                results
            },
            time_hundredths: 63,
            name: "Smelt Sticky Resin",
        },
        Recipe {
            machine: Machine::Furnace,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::PlantBall, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::SlimeBall, 2);
                results
            },
            time_hundredths: 63,
            name: "Smelt Plant Ball",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::Leaf, 8);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::PlantBall, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Plant Ball",
        },
        Recipe {
            machine: Machine::ChemicalReactor,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::CircuitBoardCoated, 1);
                components.insert(Part::Phenol, 100);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::PhenolicSubstrate, 1);
                results
            },
            time_hundredths: 500,
            name: "React Phenolic Substrate",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::StickyResin, 6);
                components.insert(Part::WoodPlanks, 3);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::CircuitBoardCoated, 3);
                results
            },
            time_hundredths: 0,
            name: "Craft Coated Circuit Board",
        },
        Recipe {
            machine: Machine::PyrolyseOven,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::Steam, 8000);
                components.insert(Part::Coal, 32);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::Phenol, 4000);
                results.insert(Part::AshSmall, 15);
                results.insert(Part::CokeDustSmall, 20);
                results
            },
            time_hundredths: (((6000.0 + 8000.0) * (4.0 / 3.0)) as u32) + 16000,
            name: "Pyrolyse Coal",
        },
        Recipe {
            machine: Machine::FluidHeater,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::Water, 6);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::Steam, 960);
                results
            },
            time_hundredths: 150,
            name: "Heat Water",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::IronWroughtPlate, 2);
                components.insert(Part::SteelPlate, 1);
                components.insert(Part::TinCable, 2);
                components.insert(Part::LvMachineCasing, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::LvMachineHull, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Lv Machine Hull",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::SteelPlate, 8);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::LvMachineCasing, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Lv Machine Casing",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::IronWroughtPlate, 2);
                components.insert(Part::AluminiumPlate, 1);
                components.insert(Part::CopperCable, 2);
                components.insert(Part::MvMachineCasing, 1);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::MvMachineHull, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Mv Machine Hull",
        },
        Recipe {
            machine: Machine::CraftingTable,
            components: {
                let mut components = BTreeMap::new();
                components.insert(Part::AluminiumPlate, 8);
                components
            },
            results: {
                let mut results = BTreeMap::new();
                results.insert(Part::MvMachineCasing, 1);
                results
            },
            time_hundredths: 0,
            name: "Craft Mv Machine Casing",
        },
    ]
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord, Copy)]
pub enum Part {
    AluminiumGearSmall,
    AluminiumIngot,
    AluminiumPlate,
    AluminiumRod,
    AntimonyDust,
    BronzeRotor,
    BronzeScrew,
    CircuitBoardGood,
    CopperAnnealedWireFine,
    CopperCable,
    CopperWire,
    CupronickelWirex2,
    Diode,
    GalliumArsenideDustSmall,
    GlassMolten,
    LeadDust,
    LvBelt,
    LvCircuit,
    LvMotor,
    LvPiston,
    LvPump,
    LvRobotArm,
    MvBelt,
    MvCircuit,
    MvMotor,
    MvPiston,
    MvPump,
    MvRobotArm,
    RubberRing,
    RubberSheet,
    SolderingAlloyDust,
    SolderingAlloyMolten,
    SteelPipe,
    SteelRodMagnetic,
    TinDust,
    TinCable,
    SteelRod,
    IronWroughtPlate,
    IronWroughtRod,
    IronWroughtGear,
    TinScrew,
    TinRotor,
    BronzePipe,
    IronRod,
    CopperWireFine,
    IronRodMagnetic,
    BronzeIngot,
    SteelIngot,
    CopperIngot,
    IronWroughtIngot,
    TinWire,
    RubberIngot,
    CircuitBoard,
    Resistor,
    RedAlloyWire,
    VacuumTube,
    IronIngot,
    CupronickelIngot,
    BronzeBolt,
    GlassTube,
    SteelBolt,
    CopperAnnealedWire,
    RedAlloyMolten,
    CarbonDust,
    Glue,
    TinBolt,
    Glass,
    TinIngot,
    NickelIngot,
    CopperAnnealedIngot,
    Hydrogen,
    Sulfur,
    Salt,
    IronDust,
    Oxygen,
    Water,
    SilverFoil,
    PhenolicSubstrate,
    GalliumArsenideDust,
    CopperFoil,
    WoodPlank,
    RubberMolten,
    LeadIngot,
    StickyResin,
    RawRubberPulp,
    SilverPlate,
    SilverIngot,
    CopperPlate,
    RedAlloyIngot,
    RedStoneDust,
    WoodPulp,
    GraphiteIngot,
    GraphiteDust,
    SlimeBall,
    PlantBall,
    Leaf,
    CircuitBoardCoated,
    Phenol,
    WoodPlanks,
    Steam,
    Coal,
    AshSmall,
    CokeDustSmall,
    MvMachineHull,
    LvMachineHull,
    SteelPlate,
    LvMachineCasing,
    MvMachineCasing,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub enum Machine {
    CraftingTable,
    CircuitAssembler,
    Extractor,
    Mixer,
    Extruder,
    Assembler,
    Wiremill,
    Lathe,
    BendingMachine,
    ElectromagneticPolarizer,
    Furnace,
    AlloySmelter,
    ChemicalReactor,
    FluidSolidifier,
    Crusher,
    Electrolyzer,
    Centrifuge,
    Compressor,
    ArcFurnace,
    PyrolyseOven,
    FluidHeater,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Recipe {
    machine: Machine,
    pub(crate) components: BTreeMap<Part, u32>,
    pub(crate) results: BTreeMap<Part, u32>,
    time_hundredths: u32,
    pub name: &'static str
}