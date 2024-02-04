/*
MANDY EXTRAS by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

/// Importing the "Itertools"
/// entity from the "itertools"
/// crate to support the
/// "vec.into_iter.unique"
/// function.
use itertools::Itertools;

/// Importing the "HashMap" data
/// structure for explicit type
/// declarations.
use std::collections::HashMap;

/// Cleans a list of category strings and removes duplicates.
/// Returns an empty list if not categories were set.
pub fn clean_raw_cats(raw_cats: &Vec<String>) -> Vec<String> {
    let mut cats: Vec<String> = Vec::new();
    if raw_cats.is_empty(){}
    else {
        let mut singled: Vec<String> = Vec::new();
        for raw_cat in raw_cats {
            let split: Vec<&str> = raw_cat.split(" ").collect();
            for split_string in split {
                singled.push(split_string.to_string());
            }
        }
        cats = singled.into_iter().unique().collect();
    }
    return cats;
}

/// Retrieves of categories from user-generated posts.
pub fn get_raw_cats(loop_ctxs: &HashMap<String, Vec<HashMap<String, String>>>) -> Vec<String> {
    let mut raw_cats: Vec<String> = Vec::new();
    for (_dir, posts) in loop_ctxs{
        for post in posts {
            if post.contains_key("categories"){
                raw_cats.push(post["categories"].clone());
            }
            else {}
        }
    }
    return raw_cats;
}

/// Transforms the base data from user-generated posts into
/// a similar data type that is ordered by categories.
pub fn build_categories(
    loop_ctxs: &HashMap<String, Vec<HashMap<String, String>>>
) -> HashMap<String, Vec<HashMap<String, String>>>{
    let mut res: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    let raw_cats: Vec<String> = get_raw_cats(loop_ctxs);
    let cleaned_cats: Vec<String> = clean_raw_cats(&raw_cats); 
    if cleaned_cats.is_empty(){}
    else {
        for cat in cleaned_cats {
            let mut post_vec: Vec<HashMap<String, String>> = Vec::new();
            for (_dir, posts) in loop_ctxs{
                for post in posts {
                    if post.contains_key("categores"){
                        let post_cats: String = post["categories"].clone();
                        if post_cats.contains(cat.as_str()){
                            post_vec.push(post.clone());
                        }
                        else {}
                    }
                }
            }
            res.insert(cat, post_vec);
        } 
    }
    res      
}