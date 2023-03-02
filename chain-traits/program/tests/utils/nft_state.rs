use chain_traits::instruction::CreateTraitConfigArgs;
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
            CreateTraitConfigArgs {
                action: chain_traits::instruction::TraitAction::Add,
                name: "Background".to_string(),
                values: vec![
                    "Vanilla Ice".to_string(),
                    "Solitary White".to_string(),
                    "Powder Puff".to_string(),
                    "Bit of Blue".to_string(),
                    "Marshmallow".to_string(),
                    "Buttercream".to_string(),
                    "Canoli Cream".to_string(),
                    "Phantom Green".to_string(),
                    "Antique White".to_string(),
                ],
            },
            CreateTraitConfigArgs {
                action: chain_traits::instruction::TraitAction::Add,
                name: "Clothes".to_string(),
                values: vec!["Banana Hazmat".to_string()],
            },
            CreateTraitConfigArgs {
                action: chain_traits::instruction::TraitAction::Add,
                name: "Eyewear".to_string(),
                values: vec!["Windsors (blackout)".to_string()],
            },
            CreateTraitConfigArgs {
                action: chain_traits::instruction::TraitAction::Add,
                name: "Face".to_string(),
                values: vec!["Blasé".to_string()],
            },
            CreateTraitConfigArgs {
                action: chain_traits::instruction::TraitAction::Add,
                name: "Fur".to_string(),
                values: vec![],
            },
            CreateTraitConfigArgs {
                action: chain_traits::instruction::TraitAction::Add,
                name: "Head".to_string(),
                values: vec!["Pyrite Crown".to_string()],
            },
            CreateTraitConfigArgs {
                action: chain_traits::instruction::TraitAction::Add,
                name: "1/1".to_string(),
                values: vec!["None".to_string()],
            },
        ]
    }
}
