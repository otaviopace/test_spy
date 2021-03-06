
#[cfg(test)]
mod tests {
    use spy::{spy, Spy};

    #[test]
    fn iterator_all_test() {
        let integers = vec![0i32, 1i32, 2i32];

        // create spy function that returns true if provided
        // argument is an even number
        let (spy_fn, spy) = spy!(|n| n % 2 == 0);

        // test call
        let res = integers.iter().all(spy_fn);

        // check Iterator::all result
        assert!(!res, "should be false");

        // take a snapshot of made calls
        let snapshot = spy.snapshot();

        // make assertions
        assert!(snapshot.called(), "should be called");
        assert!(
            snapshot.called_with(&(&1i32)),
            "should be called with 1i32 at least once"
        );
        assert!(
            !snapshot.each_called_with(&(&1i32)),
            "should be called with different arguments"
        );
        assert_eq!(snapshot.all_calls(), &vec![(&0i32), (&1i32)]);
        assert_eq!(snapshot.first_call().expect("should be Some"), &(&0i32));
        assert_eq!(snapshot.last_call().expect("should be Some"), &(&1i32));
        assert_eq!(snapshot.nth_call(1).expect("should be Some"), &(&1i32));
    }
}
