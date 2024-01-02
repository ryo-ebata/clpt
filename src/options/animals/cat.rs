use crate::systems::display;

pub fn cat() {
    let cat1 = "
      /\\_/\\
     (>o.o<)
     (     )/";

    let cat2 = "
      /\\_/\\
     (>-.-<)
    \\(     )";

    display::display(cat1, cat2);
}
