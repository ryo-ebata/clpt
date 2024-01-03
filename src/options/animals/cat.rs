use crate::systems::display_pet;

pub fn cat(name: &str) {
  let lines = 4; // The number of lines in the output

    let cat1 = "
      /\\_/\\
     (>o.o<)
     (     )/";

    let cat2 = "
      /\\_/\\
     (>-.-<)
    \\(     )";

    display_pet::display_pet(cat1, cat2, name, lines);
}
