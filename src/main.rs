use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::io::Write;
use std::process::Command;
use std::rc::Rc;
use std::sync::Arc;
// use log::{debug, error};
use data::{Part, Recipe};
use crate::data::Machine;

mod data;

fn main() {
    // env_logger::init();
    // log_panics::init();

    let recipes = data::create_recipes();
    // debug!("{:?} recipes loaded", recipes.len());

    let mut current_inventory = HashMap::new();
    current_inventory.insert(Part::AntimonyDust, 19);
    current_inventory.insert(Part::Sulfur, 44);
    current_inventory.insert(Part::Glass, 365);
    current_inventory.insert(Part::GlassMolten, 6336);
    current_inventory.insert(Part::Water, 100000);
    current_inventory.insert(Part::RubberSheet, 411);
    current_inventory.insert(Part::GalliumArsenideDust, 22);
    current_inventory.insert(Part::GalliumArsenideDustSmall, 131);
    current_inventory.insert(Part::WoodPulp, 220);
    current_inventory.insert(Part::IronDust, 6);
    current_inventory.insert(Part::GraphiteIngot, 112);
    current_inventory.insert(Part::Coal, 253);
    current_inventory.insert(Part::WoodPlanks, 319);
    current_inventory.insert(Part::IronIngot, 1708);
    current_inventory.insert(Part::IronWroughtIngot, 51);
    current_inventory.insert(Part::NickelIngot, 495);
    current_inventory.insert(Part::CopperIngot, 1846);
    current_inventory.insert(Part::RedStoneDust, 483);
    current_inventory.insert(Part::LeadIngot, 105);
    current_inventory.insert(Part::AluminiumIngot, 128);
    current_inventory.insert(Part::Salt, 268);
    //not really have these in inventory
    // 1861  RubberSheet
    // 267   SteelIngot
    // 1024  AluminiumIngot
    // 265   RedStoneDust
    // 17    NickelIngot
    // 105   SilverIngot
    // 576   SteelPlate
    // 713   TinIngot
    // 1896  Leaf
    current_inventory.entry(Part::RubberSheet).and_modify(|quantity| *quantity += 1000000);
    current_inventory.insert(Part::SteelIngot, 1000000);
    current_inventory.entry(Part::AluminiumIngot).and_modify(|quantity| *quantity += 10000000);
    current_inventory.entry(Part::RedStoneDust).and_modify(|quantity| *quantity += 265);
    current_inventory.entry(Part::NickelIngot).and_modify(|quantity| *quantity += 1000000);
    current_inventory.entry(Part::CopperIngot).and_modify(|quantity| *quantity += 10000000);
    current_inventory.insert(Part::SilverIngot, 105);
    current_inventory.insert(Part::SteelPlate, 576); //TODO: add steel plate recipe
    current_inventory.insert(Part::TinIngot, 713 + 18);
    current_inventory.insert(Part::Leaf, 1896 + 48);


    let mut final_order = HashMap::new();
    final_order.insert(Part::MvBelt, 32);
    final_order.insert(Part::LvBelt, 32);
    final_order.insert(Part::MvRobotArm, 32);
    final_order.insert(Part::LvRobotArm, 32);
    final_order.insert(Part::MvMotor, 32);
    final_order.insert(Part::LvMotor, 32);
    final_order.insert(Part::MvPiston, 32);
    final_order.insert(Part::LvPiston, 32);
    final_order.insert(Part::MvPump, 32);
    final_order.insert(Part::LvPump, 32);
    final_order.insert(Part::MvCircuit, 64);
    final_order.insert(Part::LvCircuit, 64 * 2);
    final_order.insert(Part::MvMachineHull, 64);
    final_order.insert(Part::LvMachineHull, 64);

    //make sure that we have the proper amount of parts in the inventory
    let (instructions, mut missing_ingredients) =
        calculate_total_instructions(final_order.clone(), current_inventory.clone(), &recipes);

    missing_ingredients.retain(|_, quantity| *quantity > 0);
    if !missing_ingredients.is_empty() {
        error!("missing ingredients:");
        for (part, quantity) in missing_ingredients.iter() {
            error!("{:<5} {:?}", quantity, part);
        }
    } else {
        let mut craft_tree = calculate_craft_tree(final_order, current_inventory, &recipes);

        draw_craft_tree(&craft_tree);
    }
}

fn schedual_crafts<'a>(crafts: &'a Vec<CraftUnitSource>) -> CraftSchedule {
    let current_time = 0;
    let current_schedual = CraftSchedule { machines };
}

fn draw_craft_tree(tree: &Vec<CraftUnitSource>) {
    let mut shared_recipes = vec![];
    let mut nodes = vec![];
    let mut edges = vec![];
    for node in tree.iter() {
        draw_node(node, &mut nodes, &mut edges, &mut shared_recipes);
    }
    let mut dot = "digraph G {\n".to_string();
    for (idx, (node, node_type)) in nodes.iter().enumerate() {
        dot.push_str(&format!("    {} [label=\"{}\" color={}];\n", idx, node, match node_type {
            NodeType::Recipe => "blue",
            NodeType::Inventory => "green",
            NodeType::SharedRecipe => "red",
        }));
    }
    for (from, to, label) in edges.iter() {
        dot.push_str(&format!("    {} -> {} [label=\"{}\"];\n", from, to, label));
    }
    dot.push_str("}");
    let mut child = Command::new("dot")
        .arg("-Tsvg")
        .arg("-o")
        .arg("craft_tree.svg")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::inherit())
        .spawn()
        .expect("failed to start dot");
    child.stdin.as_mut().unwrap().write_all(dot.as_bytes()).expect("failed to write to dot");
}

enum NodeType {
    Recipe,
    Inventory,
    SharedRecipe,
}

fn draw_node(node: &CraftUnitSource, node_list: &mut Vec<(String, NodeType)>, edge_list: &mut Vec<(usize, usize, String)>, shared_recipes: &mut Vec<(Rc<RefCell<(CraftUnit, Vec<BTreeMap<Part, u32>>)>>, usize)>) -> usize {
    let node_index = node_list.len();
    match node {
        CraftUnitSource::Recipe(craft_unit) => {
            let mut node_string = format!("{}x {}", craft_unit.quantity, craft_unit.recipe.name);
            node_list.push((node_string, NodeType::Recipe));
            for dep in craft_unit.dependencies.iter() {
                let dep_index = draw_node(dep, node_list, edge_list, shared_recipes);
                edge_list.push((node_index, dep_index, "".to_string()));
            }
        }
        CraftUnitSource::Inventory(part, quantity) => {
            let node_string = format!("{} {:?}", quantity, part);
            node_list.push((node_string, NodeType::Inventory));
        }
        CraftUnitSource::SharedRecipe(shared_recipe, share_idx) => {
            if let Some((shared_recipe, node_id)) = shared_recipes.iter().find(|(recipe, _)| Rc::ptr_eq(recipe, shared_recipe)) {
                let node_string = format!("{}x {}", shared_recipe.borrow().0.quantity, shared_recipe.borrow().0.recipe.name);
                node_list.push((node_string, NodeType::SharedRecipe));
                edge_list.push((node_index, *node_id, "".to_string()));
            } else {
                let node_string = format!("{}x {}", shared_recipe.borrow().0.quantity, shared_recipe.borrow().0.recipe.name);
                node_list.push((node_string, NodeType::SharedRecipe));
                shared_recipes.push((shared_recipe.clone(), shared_recipes.len()));
                for dep in shared_recipe.borrow().0.dependencies.iter() {
                    let dep_index = draw_node(dep, node_list, edge_list, shared_recipes);
                    edge_list.push((node_index, dep_index, "".to_string()));
                }
            }
        }
    }
    node_index
}

fn calculate_total_instructions(
    mut missing_parts: HashMap<Part, u32>,
    mut current_parts: HashMap<Part, u32>,
    recipies: &Vec<Recipe>,
) -> (HashMap<Recipe, u32>, HashMap<Part, u32>) {
    let mut instructions: HashMap<Recipe, u32> = HashMap::new();

    let mut resolved = false;
    while !resolved {
        resolved = true;
        //account for parts in the current inventory
        let mut to_add = HashMap::new();
        for (missing_part, missing_quantity) in missing_parts.iter_mut() {
            if missing_quantity == &0 {
                continue;
            }
            //find if we have the missing part in the current inventory
            if let Some(current_quantity) = current_parts.get_mut(missing_part) {
                if *current_quantity >= *missing_quantity {
                    *current_quantity -= *missing_quantity;
                    *missing_quantity = 0;
                } else {
                    *missing_quantity -= *current_quantity;
                    *current_quantity = 0;
                }
            }
            //find a recipe that can be used to craft the missing part
            let selected_recipe = recipies.iter().find(|recipe| {
                recipe
                    .results
                    .keys()
                    .any(|part_type| part_type == missing_part)
            });
            if selected_recipe.is_none() {
                continue;
            }
            resolved = false;
            let selected_recipe = selected_recipe.unwrap();
            let mut recipe_quantity =
                *missing_quantity / selected_recipe.results.get(missing_part).unwrap();
            while recipe_quantity * selected_recipe.results.get(missing_part).unwrap()
                < *missing_quantity
            {
                recipe_quantity += 1;
            }
            instructions
                .entry(selected_recipe.clone())
                .and_modify(|quantity| *quantity += recipe_quantity)
                .or_insert(recipe_quantity);
            //add the results to the current_parts
            for (result_type, result_amount) in selected_recipe.results.iter() {
                current_parts
                    .entry(*result_type)
                    .and_modify(|quantity| *quantity += recipe_quantity * result_amount)
                    .or_insert(recipe_quantity * result_amount);
            }
            //add all the ingredients to to_add to be added to the missing_parts
            for (ingredient_type, ingredient_amount) in selected_recipe.components.iter() {
                to_add
                    .entry(*ingredient_type)
                    .and_modify(|quantity| *quantity += recipe_quantity * ingredient_amount)
                    .or_insert(recipe_quantity * ingredient_amount);
            }
        }
        for (ingredient_type, ingredient_amount) in to_add.iter() {
            missing_parts
                .entry(*ingredient_type)
                .and_modify(|quantity| *quantity += *ingredient_amount)
                .or_insert(*ingredient_amount);
        }
    }

    (instructions, missing_parts)
}

fn calculate_craft_tree(
    order: HashMap<Part, u32>,
    mut current_inventory: HashMap<Part, u32>,
    recipies: &Vec<Recipe>,
) -> Vec<CraftUnitSource> {
    satisfy_requirements(order, &mut current_inventory, recipies)
}

#[inline]
fn stack_frame_depth() -> usize {
    let mut depth = 0;
    backtrace::trace(|_| {
        depth += 1;
        true
    });
    depth
}

fn satisfy_requirements(mut missing_parts: HashMap<Part, u32>, current_parts: &mut HashMap<Part, u32>, recipies: &Vec<Recipe>) -> Vec<CraftUnitSource> {
    // debug!("satisfy_requirements called from depth {}", stack_frame_depth());
    // debug!("missing parts: {:?}", missing_parts);
    let mut results = vec![];
    for (missing_part, missing_quantity) in missing_parts.iter_mut() {
        //find if we have the missing part in the current inventory
        if let Some(current_quantity) = current_parts.get_mut(missing_part) {
            if *current_quantity >= *missing_quantity {
                *current_quantity -= *missing_quantity;
                results.push(CraftUnitSource::Inventory(*missing_part, *missing_quantity));
                *missing_quantity = 0;
                continue;
            } else {
                *missing_quantity -= *current_quantity;
                results.push(CraftUnitSource::Inventory(*missing_part, *current_quantity));
                *current_quantity = 0;
            }
        }
    }
    missing_parts.retain(|_, quantity| *quantity > 0);
    while !missing_parts.is_empty() {
        //find a recipe that can be used to craft the missing part
        let selected_recipe = recipies.iter().find(|recipe| {
            recipe
                .results
                .keys()
                .any(|part_type| missing_parts.contains_key(part_type))
        });
        if selected_recipe.is_none() {
            panic!("could not satisfy dependency(s) of {:?}, no producing recipe(s) found and not enough in inventory", missing_parts);
            // error!("could not satisfy dependency(s) of {:?}, no producing recipe(s) found and not enough in inventory. Inserting fake Inventory dep for debugging", missing_parts);
            // for (missing_part, missing_quantity) in missing_parts.iter() {
            //     results.push(CraftUnitSource::Inventory(*missing_part, *missing_quantity));
            // }
            // missing_parts.clear();
            // break;
        }

        let selected_recipe = selected_recipe.unwrap();
        //find which missing part is satisfied by the selected recipe
        let missing_part = selected_recipe
            .results
            .keys()
            .find(|part_type| missing_parts.contains_key(part_type))
            .unwrap();

        let missing_quantity = missing_parts.get(missing_part).unwrap();

        let mut recipe_quantity =
            *missing_quantity / selected_recipe.results.get(missing_part).unwrap();
        while recipe_quantity * selected_recipe.results.get(missing_part).unwrap()
            < *missing_quantity
        {
            recipe_quantity += 1;
        }
        
        let mut craft_unit = CraftUnit {
            recipe: selected_recipe.clone(),
            quantity: recipe_quantity,
            dependencies: vec![],
        };
        
        //take away the recipe results from the missing parts if possible
        for (result_type, result_amount) in craft_unit.get_produced_parts().iter() {
            if let Some(missing_quantity) = missing_parts.get_mut(result_type) {
                if *missing_quantity >= *result_amount {
                    *missing_quantity -= *result_amount;
                } else {
                    *missing_quantity = 0;
                }
            }
        }
        craft_unit.dependencies = satisfy_requirements(craft_unit.get_dependent_parts(), current_parts, recipies);
        results.push(CraftUnitSource::Recipe(craft_unit));
        missing_parts.retain(|_, quantity| *quantity > 0);
    }
    results
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CraftUnitSource {
    Recipe(CraftUnit),
    Inventory(Part, u32),
    SharedRecipe(Rc<RefCell<(CraftUnit, Vec<BTreeMap<Part, u32>>)>>, usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CraftUnit {
    recipe: Recipe,
    quantity: u32,
    dependencies: Vec<CraftUnitSource>,
}

struct CraftSchedule {
    machines: HashMap<Machine, MachineSchedule>,
}

type MachineSchedule<'a> = Vec<(&'a Recipe, u32)>;

impl CraftUnit {
    fn divide(mut self, max_quantity: u32) -> Vec<CraftUnit> {
        if self.quantity <= max_quantity {
            return vec![self];
        }
        let mut result = vec![];
        while self.quantity > max_quantity {
            let (split_craft_unit, remaining_craft_unit) = self.split_off(max_quantity);
            result.push(split_craft_unit);
            self = remaining_craft_unit;
        }
        result.push(self);
        result
    }
    //returns (split_craft_unit, self) 
    fn split_off(mut self, quantity: u32) -> (CraftUnit, CraftUnit) {
        if self.quantity == quantity {
            return (self.clone(), CraftUnit {
                recipe: self.recipe.clone(),
                quantity: 0,
                dependencies: vec![],
            });
        }
        if self.quantity < quantity {
            panic!("tried to split off more than the quantity of the craft unit");
        }

        let mut inventory_dependencies = vec![];
        let mut recipe_dependencies = vec![];
        let mut shared_recipe_dependencies = vec![];
        for dep in self.dependencies.into_iter() {
            match dep {
                CraftUnitSource::Inventory(part, quantity) => {
                    inventory_dependencies.push(CraftUnitSource::Inventory(part, quantity));
                }
                CraftUnitSource::Recipe(recipe) => {
                    recipe_dependencies.push(CraftUnitSource::Recipe(recipe.clone()));
                }
                CraftUnitSource::SharedRecipe(shared_recipe, share_idx) => {
                    shared_recipe_dependencies.push(CraftUnitSource::SharedRecipe(
                        shared_recipe.clone(),
                        share_idx,
                    ));
                }
            }
        }
        //determine the dependencies
        let mut split_craft_unit = CraftUnit {
            recipe: self.recipe.clone(),
            quantity,
            dependencies: vec![],
        };
        self.quantity -= quantity;
        let mut unsatisfied_dependencies = split_craft_unit.get_dependent_parts();
        //first loop through the inventory dependencies and use them if possible
        for inventory_dep in inventory_dependencies.iter_mut() {
            let (part, quantity) = match inventory_dep {
                CraftUnitSource::Inventory(part, quantity) => (part, quantity),
                _ => panic!("expected inventory dependency"),
            };
            if let Some(needed_quantity) = unsatisfied_dependencies.get_mut(&part) {
                if *needed_quantity <= *quantity {
                    *needed_quantity = 0;
                    *quantity -= *needed_quantity;
                    split_craft_unit.dependencies.push(CraftUnitSource::Inventory(
                        *part,
                        *needed_quantity,
                    ));
                } else {
                    *needed_quantity -= *quantity;
                    *quantity = 0;
                    split_craft_unit.dependencies.push(CraftUnitSource::Inventory(
                        *part,
                        *quantity,
                    ));
                }
            }
        }
        //then loop through the shared recipe dependencies and use them if possible
        for shared_recipe_dep in shared_recipe_dependencies.iter_mut() {
            let (shared_recipe_usages, shared_recipe_index) = match shared_recipe_dep {
                CraftUnitSource::SharedRecipe(shared_recipe_usages, shared_recipe_index) => (shared_recipe_usages, shared_recipe_index),
                _ => panic!("expected shared recipe dependency"),
            };

            let mut shared_recipe_used = BTreeMap::new();
            for (part, quantity) in shared_recipe_usages.borrow_mut().1.get_mut(*shared_recipe_index).unwrap().iter_mut() {
                if let Some(needed_quantity) = unsatisfied_dependencies.get_mut(part) {
                    if *needed_quantity == 0 {
                        continue;
                    }
                    if *needed_quantity <= *quantity {
                        *needed_quantity = 0;
                        shared_recipe_used.insert(*part, *needed_quantity);
                        *quantity -= *needed_quantity;
                    } else {
                        *needed_quantity -= *quantity;
                        shared_recipe_used.insert(*part, *quantity);
                        *quantity = 0;
                    }
                }
            }
            shared_recipe_usages.borrow_mut().1.push(shared_recipe_used);
            split_craft_unit.dependencies.push(CraftUnitSource::SharedRecipe(
                shared_recipe_usages.clone(),
                shared_recipe_usages.borrow_mut().1.len() - 1,
            ));
        }
        //then loop through the recipe dependencies and use them if possible
        for recipe_dep in recipe_dependencies.iter_mut() {
            let craft_unit = match recipe_dep {
                CraftUnitSource::Recipe(recipe) => recipe,
                _ => panic!("expected recipe dependency"),
            };

            let mut pure_recipe_itterations = 0;
            //this will store each recipe use that has leftovers, these will then be used to create new shared recipes.
            //shared recipes that have identical leftovers will be merged
            let mut future_shared_recipes = HashMap::new();

            while map_intersection(&b_tree_map_to_hash_map(craft_unit.recipe.results.clone()), &unsatisfied_dependencies) {
                let mut leftovers = BTreeMap::new();
                for result in b_tree_map_to_hash_map(craft_unit.recipe.results.clone()).iter() {
                    if let Some(needed_quantity) = unsatisfied_dependencies.get_mut(result.0) {
                        if *needed_quantity == 0 {
                            continue;
                        }
                        if *needed_quantity <= *result.1 {
                            *needed_quantity = 0;
                            leftovers.insert(*result.0, *result.1 - *needed_quantity);
                        } else {
                            *needed_quantity -= result.1;
                        }
                    } else {
                        leftovers.insert(*result.0, *result.1);
                    }
                }

                if !map_empty(&leftovers) {
                    pure_recipe_itterations += 1;
                } else {
                    future_shared_recipes
                        .entry(leftovers)
                        .and_modify(|q| *q += 1)
                        .or_insert(1);
                }
                craft_unit.quantity -= 1;
            }

            //create all the new dependencies
            //first the pure recipe uses
            let (pure_splitoff, mut leftover_craft_unit) = craft_unit.clone().split_off(pure_recipe_itterations);
            split_craft_unit.dependencies.push(CraftUnitSource::Recipe(pure_splitoff));
            //then the shared recipe uses
            for (leftovers, quantity) in future_shared_recipes.iter() {
                let shared_splitoff;
                (shared_splitoff, leftover_craft_unit) = leftover_craft_unit.split_off(*quantity);
                //shared useage = total results - leftovers * quantity
                let mut shared_total_leftovers = b_tree_map_to_hash_map(leftovers.clone());
                map_multiply(&mut shared_total_leftovers, *quantity);
                let mut shared_usage = shared_splitoff.get_produced_parts();
                map_subtract(&mut shared_usage, &shared_total_leftovers);
                let new_shared_dep = CraftUnitSource::SharedRecipe(
                    Rc::new(RefCell::new((shared_splitoff, vec![hash_map_to_b_tree_map(shared_usage)]))),
                    0,
                );
                split_craft_unit.dependencies.push(new_shared_dep);
            }
        }
        //do some cleanup now, need to recombine the self dependencies
        let mut self_dependencies = vec![];
        self_dependencies.append(&mut inventory_dependencies);
        self_dependencies.append(&mut recipe_dependencies);
        self_dependencies.append(&mut shared_recipe_dependencies);
        self.dependencies = self_dependencies;

        //there is a possibility that the split took a full recipe and used part of it while the non-split part requires the remainder of the results of that recipe
        //check for this case by comparing the recipe requirements to the new dependencies for self
        let mut self_requirements = self.get_dependent_parts();
        let mut self_dependency_results = self.get_dependency_results();
        for (part, quantity) in self_requirements.iter() {
            if let Some(dependency_quantity) = self_dependency_results.get(part) {
                if *dependency_quantity < *quantity {
                    let mut difference = *quantity - *dependency_quantity;
                    //there is a missing dependency, loop through the dependencies of the split off part looking for a suitable shared recipe
                    for dep in split_craft_unit.dependencies.iter_mut() {
                        if let CraftUnitSource::SharedRecipe(shared_recipe, share_idx) = dep {
                            let shared_recipe_leftovers = get_shared_craft_leftovers(shared_recipe);
                            if let Some(share_quantity) = shared_recipe_leftovers.get(part) {
                                if *share_quantity >= difference {
                                    let mut shared_recipe_used = BTreeMap::new();
                                    shared_recipe_used.insert(*part, difference);
                                    shared_recipe.borrow_mut().1.push(shared_recipe_used);
                                    self.dependencies.push(CraftUnitSource::SharedRecipe(
                                        shared_recipe.clone(),
                                        shared_recipe.borrow_mut().1.len() - 1,
                                    ));
                                    difference = 0;
                                    break;
                                } else {
                                    let mut shared_recipe_used = BTreeMap::new();
                                    shared_recipe_used.insert(*part, *share_quantity);
                                    shared_recipe.borrow_mut().1.push(shared_recipe_used);
                                    self.dependencies.push(CraftUnitSource::SharedRecipe(
                                        shared_recipe.clone(),
                                        shared_recipe.borrow_mut().1.len() - 1,
                                    ));
                                    difference -= *share_quantity;
                                }
                            }
                        }
                    }
                    if difference > 0 {
                        panic!("could not find a shared recipe to satisfy the missing dependency");
                    }
                }
            }
        }
        (split_craft_unit, self)
    }
    fn get_dependent_parts(&self) -> HashMap<Part, u32> {
        let mut result = HashMap::new();
        for component in self.recipe.components.iter() {
            result
                .entry(*component.0)
                .and_modify(|q| *q += component.1 * self.quantity)
                .or_insert(component.1 * self.quantity);
        }
        result
    }
    fn get_produced_parts(&self) -> HashMap<Part, u32> {
        let mut result = HashMap::new();
        for (part, quantity) in self.recipe.results.iter() {
            result
                .entry(*part)
                .and_modify(|q| *q += quantity * self.quantity)
                .or_insert(quantity * self.quantity);
        }
        result
    }
    fn get_dependency_results(&self) -> HashMap<Part, u32> {
        let mut result = HashMap::new();
        for dep in self.dependencies.iter() {
            match dep {
                CraftUnitSource::Recipe(craft_unit) => {
                    for (part, quantity) in craft_unit.get_produced_parts().iter() {
                        result
                            .entry(*part)
                            .and_modify(|q| *q += quantity * craft_unit.quantity)
                            .or_insert(*quantity * craft_unit.quantity);
                    }
                }
                CraftUnitSource::Inventory(part, quantity) => {
                    result
                        .entry(*part)
                        .and_modify(|q| *q += quantity)
                        .or_insert(*quantity);
                }
                CraftUnitSource::SharedRecipe(shared_recipe, share_idx) => {
                    for (part, quantity) in shared_recipe.borrow_mut().1.get(*share_idx).unwrap().iter() {
                        result
                            .entry(*part)
                            .and_modify(|q| *q += quantity)
                            .or_insert(*quantity);
                    }
                }
            }
        }
        result
    }
}


//checks if 2 maps have a common key with both values > 0
fn map_intersection<K>(map1: &HashMap<K, u32>, map2: &HashMap<K, u32>) -> bool
where
    K: Eq + Hash,
{
    for (key, quantity) in map1.iter() {
        if let Some(other_quantity) = map2.get(key) {
            if *quantity > 0 && *other_quantity > 0 {
                return true;
            }
        }
    }
    false
}

//multiplies all values in a map by a factor
fn map_multiply<K>(map: &mut HashMap<K, u32>, factor: u32) {
    for (_, quantity) in map.iter_mut() {
        *quantity *= factor;
    }
}

//subtracts the values of map2 from map1
fn map_subtract<K>(map1: &mut HashMap<K, u32>, map2: &HashMap<K, u32>)
where
    K: Eq + Hash,
{
    for (key, quantity) in map2.iter() {
        if let Some(map1_quantity) = map1.get_mut(key) {
            if *map1_quantity > *quantity {
                *map1_quantity -= *quantity;
            } else {
                panic!("tried to subtract a map from another map with a value greater than the original");
            }
        } else {
            panic!("tried to subtract a map from another map with different keys");
        }
    }
}

//checks if a map is empty(contains only 0 values)
fn map_empty<'a, K: 'a, H: 'a>(map: &'a H) -> bool
where
    K: Eq + Hash,
    &'a H: IntoIterator<Item=(&'a K, &'a u32)>,
{
    for (_, quantity) in map.into_iter() {
        if *quantity > 0 {
            return false;
        }
    }
    true
}

fn b_tree_map_to_hash_map<K, V>(map: BTreeMap<K, V>) -> HashMap<K, V>
where
    K: Eq + Hash,
{
    map.into_iter().collect()
}

fn hash_map_to_b_tree_map<K, V>(map: HashMap<K, V>) -> BTreeMap<K, V>
where
    K: Ord,
{
    map.into_iter().collect()
}

impl CraftUnitSource {
    fn get_produced_parts(&self) -> HashMap<Part, u32> {
        match self {
            CraftUnitSource::Recipe(recipe) => {
                recipe.get_produced_parts()
            }
            CraftUnitSource::Inventory(part, quantity) => {
                let mut result = HashMap::new();
                result
                    .entry(*part)
                    .and_modify(|q| *q += quantity)
                    .or_insert(*quantity);
                result
            }
            CraftUnitSource::SharedRecipe(shared_recipe, share_idx) => {
                b_tree_map_to_hash_map(shared_recipe.borrow_mut().1.get(*share_idx as usize).unwrap().clone())
            }
        }
    }
}

fn get_shared_craft_leftovers(shared_recipe: &mut Rc<RefCell<(CraftUnit, Vec<BTreeMap<Part, u32>>)>>)
                              -> HashMap<Part, u32>
{
    //first add all the recipe results
    let mut result = shared_recipe.borrow_mut().0.get_produced_parts();
    //then take away all the used results recorded in the shared recipe
    for used_result in shared_recipe.borrow_mut().1.iter() {
        for (part, quantity) in used_result.iter() {
            result
                .entry(*part)
                .and_modify(|q| *q -= quantity);
        }
    }
    result
}