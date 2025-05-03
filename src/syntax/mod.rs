use std::collections::HashSet;

pub mod config;
pub mod node;

// https://drafts.csswg.org/css-contain-3/#container-rule
// The keywords `none`, `and`, `not`, and `or` are excluded from the <custom-ident> above.
const NON_CONTAINER_NAME_KEYWORDS: HashSet<&str> = HashSet::from(["none", "and","not", "or"]);

pub struct AtContainerParser {}

 impl AtContainerParser {

        pub fn prelude() -> () {
            const children = this.createList();
        }


       fn block(nested: bool) -> () {

        }

}
