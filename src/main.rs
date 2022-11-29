use flatbuffers;

// import the generated code
#[allow(dead_code, unused_imports)]
#[path = "./schema_generated.rs"]
mod schema;


fn main() {
    let mut sword_builder = flatbuffers::FlatBufferBuilder::new();
    let sword_name = sword_builder.create_string("Sword");
    let sword = schema::Weapon::create(
        &mut sword_builder,
        &schema::WeaponArgs {
            name: Some(sword_name),
            damage: 10,
            two_handed: false,
        },
    );
    sword_builder.finish(sword, None);
    let sword_buffer = sword_builder.finished_data();


    let mut shield_builder = flatbuffers::FlatBufferBuilder::new();
    let shield_name = shield_builder.create_string("Shield");
    let shield = schema::Weapon::create(
        &mut shield_builder,
        &schema::WeaponArgs {
            name: Some(shield_name),
            damage: 2,
            two_handed: true,
        },
    );
    shield_builder.finish(shield, None);
    let shield_buffer = shield_builder.finished_data();


    // Lets decode our buffers
    let sword_decoded = flatbuffers::root::<schema::Weapon>(&sword_buffer).unwrap();
    println!("{:#?}", sword_decoded);

    let shield_decoded = flatbuffers::root::<schema::Shield>(&shield_buffer).unwrap();
    println!("{:#?}", shield_decoded);

    // This should fail:
    let sword_decoded_failure = flatbuffers::root::<schema::Weapon>(&shield_buffer).unwrap();
    println!("{:#?}", sword_decoded_failure);
}
