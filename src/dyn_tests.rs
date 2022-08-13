struct HasSingleTrait {}

struct HasMultipleTraits {}

trait FirstTrait {}

trait SecondTrait {}

impl FirstTrait for HasSingleTrait {}

impl FirstTrait for HasMultipleTraits {}

impl SecondTrait for HasSingleTrait {}

/// this is meant to show if it is possible to require multiple trait bounds for dynamic dispatch
fn can_i_habe_the_trait_boss() {
    let single_trait_struct = HasSingleTrait {};
    let multiple_trait_struct = HasSingleTrait {};

    let requires_single_trait: Vec<Box<dyn FirstTrait>> = vec![Box::new(single_trait_struct)];

    let single_trait_struct = HasSingleTrait {};
    let requires_single_trait: Vec<Box<dyn FirstTrait>> = vec![
        Box::new(single_trait_struct),
        Box::new(multiple_trait_struct),
    ];

    let single_trait_struct = HasSingleTrait {};
    let multiple_trait_struct = HasSingleTrait {};
    // let requires_single_trait: Vec<Box<dyn FirstTrait, SecondTrait>> =
        // vec![Box::new(multiple_trait_struct)];
}
