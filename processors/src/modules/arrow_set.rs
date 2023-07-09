/*
MANDY by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the method
/// to split strings into
/// a vector of strings from
/// the "coutils" crate.
use coutils::clean_split;

/// Importing Mandy's error
/// struct.
use merrors::MandyError;

/// A structure to represent
/// the fields of an arrow set.
pub struct ArrowSet{
    pub flag: String,
    pub set: Vec<String>
}

/// Implementing generic
/// methods for the arrow
/// set data structure.
impl ArrowSet {

    /// Implements a convenience
    /// method for creating new
    /// instances of the data
    /// structure.
    pub fn new(
        flag: &String,
        set: &Vec<String>
    ) -> ArrowSet {
        return ArrowSet{
            flag: flag.to_owned(),
            set: set.to_owned(),
        }
    }

}

/// Parses a string into an arrow set string into the arrow set data structure.
/// Returns an error if this fails.
pub fn parse_arrow_set(subject: &String) -> Result<ArrowSet, MandyError> {
    let arrow_components: Vec<String> = clean_split(
        subject,
        &String::from("=>")
    );
    if arrow_components.len() == 1 || arrow_components.is_empty(){
        let msg: String = String::from("Arrow not found in expression.");
        return Err::<ArrowSet, MandyError>(
            MandyError::new(&msg.to_string())
        );
    }
    else {
        let arrow_set_items: Vec<String> = clean_split(
            &arrow_components[1],
            &String::from("|")
        );
        if arrow_set_items.is_empty(){
            let msg: String = String::from("Nothing on the right side of the arrow.");
            return Err::<ArrowSet, MandyError>(
                MandyError::new(&msg.to_string())
            );

        }
        else {
            let arrow_set: ArrowSet = ArrowSet::new(&arrow_components[0], &arrow_set_items);
            return Ok(arrow_set);
        }
    }
}
