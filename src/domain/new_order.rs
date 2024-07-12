use crate::domain::item::Item;
use crate::domain::nat::Nat;

// We don't want negative numbers when creating a new order even though the database has signed integers
pub struct NewOrder {
    pub table_no: Nat,
    pub item: Item,
    pub quantity: Nat,
    pub preparation_time: Nat,
}
