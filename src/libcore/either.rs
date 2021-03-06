#[doc = "A type that represents one of two alternatives"];

#[doc = "The either type"]
enum t<T, U> {
    left(T),
    right(U)
}

fn either<T, U, V>(f_left: fn(T) -> V,
                   f_right: fn(U) -> V, value: t<T, U>) -> V {
    #[doc = "
    Applies a function based on the given either value

    If `value` is left(T) then `f_left` is applied to its contents, if `value`
    is right(U) then `f_right` is applied to its contents, and the result is
    returned.
    "];

    alt value { left(l) { f_left(l) } right(r) { f_right(r) } }
}

fn lefts<T: copy, U>(eithers: [t<T, U>]) -> [T] {
    #[doc = "Extracts from a vector of either all the left values"];

    let mut result: [T] = [];
    for elt: t<T, U> in eithers {
        alt elt { left(l) { result += [l]; } _ {/* fallthrough */ } }
    }
    ret result;
}

fn rights<T, U: copy>(eithers: [t<T, U>]) -> [U] {
    #[doc = "Extracts from a vector of either all the right values"];

    let mut result: [U] = [];
    for elt: t<T, U> in eithers {
        alt elt { right(r) { result += [r]; } _ {/* fallthrough */ } }
    }
    ret result;
}

fn partition<T: copy, U: copy>(eithers: [t<T, U>])
    -> {lefts: [T], rights: [U]} {
    #[doc = "
    Extracts from a vector of either all the left values and right values

    Returns a structure containing a vector of left values and a vector of
    right values.
    "];

    let mut lefts: [T] = [];
    let mut rights: [U] = [];
    for elt: t<T, U> in eithers {
        alt elt { left(l) { lefts += [l]; } right(r) { rights += [r]; } }
    }
    ret {lefts: lefts, rights: rights};
}

pure fn flip<T: copy, U: copy>(eith: t<T, U>) -> t<U, T> {
    #[doc = "Flips between left and right of a given either"];

    alt eith {
      right(r) { left(r) }
      left(l) { right(l) }
    }
}

pure fn to_result<T: copy, U: copy>(eith: t<T, U>) -> result::t<U, T> {
    #[doc = "
    Converts either::t to a result::t

    Converts an `either` type to a `result` type, making the \"right\" choice
    an ok result, and the \"left\" choice a fail
    "];

    alt eith {
      right(r) { result::ok(r) }
      left(l) { result::err(l) }
    }
}

pure fn is_left<T, U>(eith: t<T, U>) -> bool {
    #[doc = "Checks whether the given value is a left"];

    alt eith { left(_) { true } _ { false } }
}

pure fn is_right<T, U>(eith: t<T, U>) -> bool {
    #[doc = "Checks whether the given value is a right"];

    alt eith { right(_) { true } _ { false } }
}

#[test]
fn test_either_left() {
    let val = left(10);
    fn f_left(&&x: int) -> bool { x == 10 }
    fn f_right(&&_x: uint) -> bool { false }
    assert (either(f_left, f_right, val));
}

#[test]
fn test_either_right() {
    let val = right(10u);
    fn f_left(&&_x: int) -> bool { false }
    fn f_right(&&x: uint) -> bool { x == 10u }
    assert (either(f_left, f_right, val));
}

#[test]
fn test_lefts() {
    let input = [left(10), right(11), left(12), right(13), left(14)];
    let result = lefts(input);
    assert (result == [10, 12, 14]);
}

#[test]
fn test_lefts_none() {
    let input: [t<int, int>] = [right(10), right(10)];
    let result = lefts(input);
    assert (vec::len(result) == 0u);
}

#[test]
fn test_lefts_empty() {
    let input: [t<int, int>] = [];
    let result = lefts(input);
    assert (vec::len(result) == 0u);
}

#[test]
fn test_rights() {
    let input = [left(10), right(11), left(12), right(13), left(14)];
    let result = rights(input);
    assert (result == [11, 13]);
}

#[test]
fn test_rights_none() {
    let input: [t<int, int>] = [left(10), left(10)];
    let result = rights(input);
    assert (vec::len(result) == 0u);
}

#[test]
fn test_rights_empty() {
    let input: [t<int, int>] = [];
    let result = rights(input);
    assert (vec::len(result) == 0u);
}

#[test]
fn test_partition() {
    let input = [left(10), right(11), left(12), right(13), left(14)];
    let result = partition(input);
    assert (result.lefts[0] == 10);
    assert (result.lefts[1] == 12);
    assert (result.lefts[2] == 14);
    assert (result.rights[0] == 11);
    assert (result.rights[1] == 13);
}

#[test]
fn test_partition_no_lefts() {
    let input: [t<int, int>] = [right(10), right(11)];
    let result = partition(input);
    assert (vec::len(result.lefts) == 0u);
    assert (vec::len(result.rights) == 2u);
}

#[test]
fn test_partition_no_rights() {
    let input: [t<int, int>] = [left(10), left(11)];
    let result = partition(input);
    assert (vec::len(result.lefts) == 2u);
    assert (vec::len(result.rights) == 0u);
}

#[test]
fn test_partition_empty() {
    let input: [t<int, int>] = [];
    let result = partition(input);
    assert (vec::len(result.lefts) == 0u);
    assert (vec::len(result.rights) == 0u);
}

//
// Local Variables:
// mode: rust
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
//
