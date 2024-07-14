#[derive(Debug)]
pub struct Item(String);

/*
 * We need to ensure what a valid item it in this module
 * Additionally check for common errors of emptyness and containing forbidden characters
 * Key here is that we don't just have one source of truth which is why I have a separate list of valid items
 * which are ensured to contain the same valid items.
 * Two sources of truth reduce the chance of accidental changes to the valid items.
 */

// Valid menu items used only in this module
const VALID_ITEMS: [&str; 4] = ["hamburger", "fries", "cola", "water"];

impl Item {
    // Returns an instance of `Item` if the input satisfies validation
    pub fn parse(s: String) -> Result<Item, String> {
        // Checking if empty
        let is_empty_or_whitespace = s.trim().is_empty();

        // Checking for forbidden characters
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        // Checking if the item is in the valid item list
        let valid_item = VALID_ITEMS.iter().any(|&g| g == s);

        if is_empty_or_whitespace || contains_forbidden_characters {
            Err(format!(
                "{} item is empty or contains forbidden characters.",
                s
            ))
        } else if !valid_item {
            Err(format!("{} item is not in the valid item list.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Item {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::menu_items::MENU_ITEMS;
    use crate::domain::{item::VALID_ITEMS, Item};
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_valid_item_is_parsed_successfully() {
        let item = "hamburger".to_string();
        assert_ok!(Item::parse(item));
    }

    #[test]
    fn no_partial_match_for_valid_item() {
        let item = "hamburge".to_string();
        assert_err!(Item::parse(item));
    }

    #[test]
    fn valid_item_with_additions_is_rejected() {
        for addition in &[" ", "1", "fries"] {
            let mut item = "hamburger".to_string();
            item.push_str(addition);
            assert_err!(Item::parse(item));
        }
    }

    #[test]
    fn all_menu_items_are_parsed_successfully() {
        for menu_item in MENU_ITEMS {
            assert_ok!(Item::parse(menu_item.to_string()));
        }
    }

    #[test]
    fn all_valid_items_are_contained_in_menu_items_constant() {
        assert!(VALID_ITEMS.iter().all(|item| MENU_ITEMS.contains(item)));
    }

    #[test]
    fn whitespace_only_items_are_rejected() {
        let item = " ".to_string();
        assert_err!(Item::parse(item));
    }

    #[test]
    fn empty_string_is_rejected() {
        let item = "".to_string();
        assert_err!(Item::parse(item));
    }

    #[test]
    fn items_containing_an_invalid_character_are_rejected() {
        for item in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let item = item.to_string();
            assert_err!(Item::parse(item));
        }
    }
}
