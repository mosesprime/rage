//*
* Demo for RAGE
*/

main() {

    { // explicit assignment & unsigned integer
        a u32 = 0
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

    { // scoping
        x mut u32 = 0
        { // read-only "borrow" while "downscoping"
            y = x + 1
            assert!(y == 1)
            // var 'y' scoped to this block and is dropped here
        }
        move x { // move "consumes" the var 'x'. when this block ends, 'x' is dropped.
            x += 2
            assert!(x == 2)
        }

        p mut i32 = 0
        p = move p { // var 'p' is recycled here. 
            p -= 5
            return p
        }
        assert!(p == -5)

        { // function calls can "borrow" or "consume"
            is_true(b %bool) bool { // this consumes 'b', explicitly pass-by-value
                return b
            }
            q = false
            assert!(is_true(q) == false) // 'q' no longer acessable

            add(n1 *u32, n2 *u32) u32 { // this borrows 'n1' and 'n2', takes pointer to value
                return %n1 + %n2 // explicit use value of pointer
            }
            s = 1; t = 2 // semi-colon seperator when not using newline
            assert!(add(&s, &t) == s+t) // 's' and 't' acessabel after 'add()', pass a reference

            sub(n1 i32, n2 i32) i32 { // likely optimizes to pass-by-ref, but since thats unsure, consumes 'n1' and 'n2'
                return n1 - n2 // if optimized to pass-by-ref, implicitly uses value
            }
            h = -2; i isize = 5
            assert!(sub(i, h) == 7) // since 'h' and 'i' might be consumed, compiler assumes they are no longer accessable
        }
    }
}
