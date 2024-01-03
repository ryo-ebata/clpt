use crate::systems::display_pet;

pub fn dog(name: &str) {
    let lines = 6; // The number of lines in the output

    let dog1 = "
 / \\__
(    @\\___
/          O
/    (_____/
=====/ U";

    let dog2 = "
 / \\__
(    -\\___
/          O
/    (_____/
=====/U";

    display_pet::display_pet(dog1, dog2, name, lines);
}
