// ANCHOR: entity_struct
struct Entity(u64);
// ANCHOR_END: entity_struct

fn main() {
    let entity = Entity(14);
    println!("entity: {}", entity.0);
}
