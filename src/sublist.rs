use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
fn is_sublist<T: PartialEq>(superlist: &[T], sublist: &[T]) -> bool {
    let mut index = 0;

    if sublist.is_empty() {
        return true;
    }

    while index <= superlist.len() - sublist.len() {
        let slice = &superlist[index..index + sublist.len()];
        if slice.eq(sublist) {
            return true;
        }
        index += 1;
    }

    false
}
// When I almost completed this task and I was trying to solve the problem with displaying list values on a console,
// I found the "windows" method. So I decided to use both functions here.
// On exercism i'll send only v2 version.
fn is_sublist_v2<T: PartialEq>(superlist: &[T], sublist: &[T]) -> bool {
    if sublist.is_empty() {
        return true;
    }

    superlist.windows(sublist.len()).any(|x| x == sublist)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.iter().eq(_second_list.iter()) {
        return Comparison::Equal;
    }

    match _first_list.len().cmp(&_second_list.len()) {
        Ordering::Greater => {
            if is_sublist(_first_list, _second_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Less => {
            if is_sublist_v2(_second_list, _first_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Equal => Comparison::Unequal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_lists() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Equal;
        assert_eq!(output, expected);
    }
    #[test]
    fn empty_list_within_non_empty_list() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }
    #[test]
    fn non_empty_list_contains_empty_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }
    #[test]
    fn list_equals_itself() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Equal;
        assert_eq!(output, expected);
    }
    #[test]
    fn different_lists() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[2, 3, 4];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
    #[test]
    fn false_start() {
        let list_one: &[i32] = &[1, 2, 5];
        let list_two: &[i32] = &[0, 1, 2, 3, 1, 2, 5, 6];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }
    #[test]
    fn consecutive() {
        let list_one: &[i32] = &[1, 1, 2];
        let list_two: &[i32] = &[0, 1, 1, 1, 2, 1, 2];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }
    #[test]
    fn sublist_at_start() {
        let list_one: &[i32] = &[0, 1, 2];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }
    #[test]
    fn sublist_in_middle() {
        let list_one: &[i32] = &[2, 3, 4];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }
    #[test]
    fn sublist_at_end() {
        let list_one: &[i32] = &[3, 4, 5];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }
    #[test]
    fn at_start_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[0, 1, 2];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }
    #[test]
    fn in_middle_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }
    #[test]
    fn at_end_of_superlist() {
        let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
        let list_two: &[i32] = &[3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Superlist;
        assert_eq!(output, expected);
    }
    #[test]
    fn first_list_missing_element_from_second_list() {
        let list_one: &[i32] = &[1, 3];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
    #[test]
    fn second_list_missing_element_from_first_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[1, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
    #[test]
    fn first_list_missing_additional_digits_from_second_list() {
        let list_one: &[i32] = &[1, 2];
        let list_two: &[i32] = &[1, 22];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
    #[test]
    fn order_matters_to_a_list() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[3, 2, 1];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
    #[test]
    fn same_digits_but_different_numbers() {
        let list_one: &[i32] = &[1, 0, 1];
        let list_two: &[i32] = &[10, 1];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }
}
