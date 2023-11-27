//*
* Demo for RAGE
*/

main() {

    { // explicit assignment & unsigned integer
        a: u32 = 0
        assert!(a == 0)
    }

    { // implicit assignment & floating point number
        pi = 3.14
        assert!(pi == 3.14)
    }

    { // mutability & signed integer
        c mut = 69
        c = -420
        assert!(c == -420)
    }

    { // string slice
        s = "Hi Mom!"
        assert!(s.len() == 7)
    }

    { // increment & decrement
        n mut = 0
        n += 1
        n -= 2
        assert!(n == -1)
    }

    { // arithmetic
        a = 5
        b = a + 5
        assert!(b == 10)
    }
}
