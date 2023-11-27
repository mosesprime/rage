//*
* Demo for RAGE
*/

main() {

    { // explicit assignment
        a: u8 = 0
        assert!(a == 0)
    }

    { // implicit assignment
        pi = 3.14
        assert!(pi == 3.14)
    }

    {
        c mut = "foo"
        c = "bar"
        assert!(c == "bar")
    }
}
