use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct ToDoArgs {
    #[clap(subcommand)]
    //Declaring first argument to be subcommand for others and be a type of entitytype
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    //Declaring other commands when two of them are holding type Element, and the last one is just Show type
    Add(Element),
    Remove(Id),
    Show,
}

#[derive(Debug, Args)]
pub struct Element {
    //Declaring how element struct looks like
    pub element: String,
}

#[derive(Debug, Args)]
pub struct Id {
    pub id: u32,
}
