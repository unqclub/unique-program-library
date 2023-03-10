use std::collections::HashMap;

use chain_traits::{
    instruction::{CreateTraitArgs, CreateTraitConfigArgs, TraitAction, TraitValueAction},
    state::{AvailableTrait, TraitConfig, TraitConfigKey},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct UriMetadata {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<Trait>,
}
#[derive(Serialize, Debug, Deserialize)]

pub struct Trait {
    pub trait_type: String,
    pub value: String,
}
#[derive(Serialize, Debug, Deserialize)]

pub struct Property {
    category: Option<String>,
    files: Vec<File>,
}
#[derive(Serialize, Debug, Deserialize)]

pub struct File {
    uri: String,
    ttype: String,
}

impl UriMetadata {
    pub fn get_traits() -> Vec<CreateTraitConfigArgs> {
        vec![
            // CreateTraitConfigArgs {
            //     name: "Background".to_string(),
            //     values: vec![
            //         TraitValueAction {
            //             name: "Vanilla Ice".to_string(),
            //             action: TraitAction::Add,
            //         },
            //         TraitValueAction {
            //             name: "Ruby Red".to_string(),
            //             action: TraitAction::Add,
            //         },
            //         TraitValueAction {
            //             name: "Marshmallow".to_string(),
            //             action: TraitAction::Add,
            //         },
            //     ],
            // },
            // CreateTraitConfigArgs {
            //     name: "Clothes".to_string(),
            //     values: vec![TraitValueAction {
            //         name: "Nice Overalls".to_string(),
            //         action: TraitAction::Add,
            //     }],
            // },
            // CreateTraitConfigArgs {
            //     name: "Eyewear".to_string(),
            //     values: vec![TraitValueAction {
            //         name: "Nouns".to_string(),
            //         action: TraitAction::Add,
            //     }],
            // },
            // CreateTraitConfigArgs {
            //     name: "Face".to_string(),
            //     values: vec![TraitValueAction {
            //         action: TraitAction::Add,
            //         name: "Smirk".to_string(),
            //     }],
            // },
            // CreateTraitConfigArgs {
            //     name: "Fur".to_string(),
            //     values: vec![TraitValueAction {
            //         action: TraitAction::Add,
            //         name: "Eggnog".to_string(),
            //     }],
            // },
            // CreateTraitConfigArgs {
            //     name: "Head".to_string(),
            //     values: vec![TraitValueAction {
            //         action: TraitAction::Add,
            //         name: "Spiky Hair".to_string(),
            //     }],
            // },
            // CreateTraitConfigArgs {
            //     name: "1/1".to_string(),
            //     values: vec![TraitValueAction {
            //         action: TraitAction::Add,
            //         name: "None".to_string(),
            //     }],
            // },
        ]
    }

    pub fn map_traits() -> HashMap<TraitConfigKey, HashMap<u8, AvailableTrait>> {
        let traits = Self::get_traits();
        let mut trait_map: HashMap<TraitConfigKey, HashMap<u8, AvailableTrait>> = HashMap::new();

        // traits
        //     .iter()
        //     .enumerate()
        //     .for_each(|(name_index, trait_info)| {
        //         let mut trait_values: HashMap<u8, AvailableTrait> = HashMap::new();

        //         trait_info.values.iter().enumerate().for_each(|(index, v)| {
        //             trait_values.insert(
        //                 index as u8,
        //                 AvailableTrait {
        //                     value: v.name.clone(),
        //                     is_active: true,
        //                 },
        //             );
        //         });

        //         trait_map.insert(
        //             TraitConfigKey {
        //                 name: trait_info.name.clone(),
        //                 id: name_index as u8,
        //             },
        //             trait_values,
        //         );
        //     });

        trait_map
    }

    pub fn map_to_args(&self, trait_config: TraitConfig) -> Vec<CreateTraitArgs> {
        let mut trait_args: Vec<CreateTraitArgs> = Vec::new();

        self.attributes.iter().for_each(|attr| {
            let found_trait = trait_config
                .available_traits
                .iter()
                .find(|t| t.0.name == attr.trait_type)
                .unwrap();
            trait_args.push(CreateTraitArgs {
                name: found_trait.0.id,
                value: *found_trait
                    .1
                    .iter()
                    .find(|trait_value| trait_value.1.value == attr.value)
                    .unwrap()
                    .0,
            })
        });
        trait_args
    }
}
