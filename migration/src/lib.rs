pub use sea_orm_migration::prelude::*;

mod m20221213_161316_create_user_table;
mod m20221216_141436_user_table_add_email;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221213_161316_create_user_table::Migration),
            Box::new(m20221216_141436_user_table_add_email::Migration),
        ]
    }
}
