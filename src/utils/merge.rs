pub mod any {
    pub fn overwrite<T>(left: &mut T, right: T) {
        *left = right;
    }

    pub fn overwrite_default<T: PartialEq>(left: &mut T, right: T, default: T) {
        if *left == default {
            *left = right;
        }
    }
}

pub mod string {
    pub fn overwrite_not_empty(left: &mut String, right: &str) {
        if !right.trim().is_empty() {
            *left = right.to_string();
        }
    }
}

pub mod indexmap {
    use std::hash::Hash;
    use indexmap::IndexMap;

    pub fn overwrite<K: Eq + Hash, V>(left: &mut IndexMap<K, V>, right: IndexMap<K, V>) {
        left.extend(right)
    }
}

pub mod skin_element {
    use crate::generic::elements::SkinElement;

    pub fn overwrite_if_data<T: SkinElement>(left: &mut T, right: T) {
        if right.has_data() {
            *left = right
        }
    }
}

pub mod skin {
    use std::collections::HashMap;

    use crate::traits::KeymodeInvariant;

    pub fn overwrite_keymode<T: KeymodeInvariant>(left: &mut Vec<T>, right: Vec<T>) {
        let mut map: HashMap<_, T> = left
            .drain(..)
            .map(|item| (item.get_keymode(), item))
            .collect();

        for item in right {
            map.insert(item.get_keymode(), item);
        }

        *left = map.into_values().collect();
        left.sort_by_key(|item| item.get_keymode());
    }
}
