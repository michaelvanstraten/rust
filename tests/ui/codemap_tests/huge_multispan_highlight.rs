//@ revisions: ascii unicode
//@ compile-flags: --color=always
//@[ascii] compile-flags: --error-format=human
//@[unicode] compile-flags: -Zunstable-options=yes --error-format=human-unicode
//@ ignore-windows
fn main() {
    let _ = match true {
        true => (
            // last line shown in multispan header
























































































        ),
        false => "





















        ",
    };
    let _ = match true {
        true => (

        1 // last line shown in multispan header
























































































        ),
        false => "


        1 last line shown in multispan



















        ",
    };
}
