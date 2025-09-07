// When data is bound by the same name immutably, it also freezes.
// Frozen data can't be modified until the immutable binding goes out of scope:

fn main() {
    let mut _mutable_integer = 69;
    {
        //shadowing with a immutable var
        let _mutable_integer = _mutable_integer;

        //_mutable_integer has been frozen in this scope and thus cant mutate no mo
        // _mutable_integer = 420;

        /*
        error[E0384]: cannot assign twice to immutable variable `_mutable_integer`
        --> chapter04/src/bin/freezing.rs:11:9
            |
        8  |         let _mutable_integer = _mutable_integer;
            |             ---------------- first assignment to `_mutable_integer`
        ...
        11 |         _mutable_integer = 420;
            |         ^^^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
            |
        help: consider making this binding mutable
            |
        8  |         let mut _mutable_integer = _mutable_integer;
            |             +++

        For more information about this error, try `rustc --explain E0384`.
        */
        //frozen _mutable_integer goes out of scope after this line
    }
    //this is fine cos the forzen var is out of scope, thus the var is not frozen no mo
    _mutable_integer = 69420;
}
