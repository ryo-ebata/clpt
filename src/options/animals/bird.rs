use crate::systems::display_pet;

pub fn bird(name: &str) {
    let lines = 6; // The number of lines in the output

    let bird1 = "
    \\\\
    (o>
\\\\_//|
 \\_/_)
   ⊥⊥";

    let bird2 = "
   \\\\
   (o>
\\\\_||)
 \\_\\_)
   ⊥⊥";

    display_pet::display_pet(bird1, bird2, name, lines);
}
