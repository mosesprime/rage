//*
* Demo for RAGE
*/

main fn() {

    { // explicit assignment & unsigned integer
        a u32 = 0
        assert(a == 0)
    }

    { // implicit assignment & floating point number
        pi = 3.14
        assert(pi == 3.14)
    }

    { // mutability & signed integer
        c mut i32 = 69
        c = -420
        assert(c == -420)
    }

    { // string slice
        s = "Hi Mom!"
        assert(s.len() == 7)
    }

    { // increment & decrement
        n mut = 0
        n += 1
        n -= 2
        assert(n == -1)
    }

    { // arithmetic
        a = 5
        b = a + 5
        assert(b == 10)
    }

    { // scoping
        x mut u32 = 0
        { // read-only "borrow" of value
            y = %x + 1
            assert(y == 1)
            // var 'y' scoped to this block and is dropped here
        }

        { // 
            x += 2
            assert(x == 2)
        }

        { // pass-by-value arguments
            is_true(b %bool) bool { // explicitly pass-by-value
                return b
            }
            q = false
            assert(is_true(%q) == false) // 'q' is untouched and still accessable
        }

        { // mutable pass-by-reference // TODO: fix this
            inc_by_2(n mut *u32) {
                // n += 2 pointer arithmetic disallowed for safety
                %n += 2 // incriment the value, same as *n += 2
            }
            x mut = 5
            inc_by_2(mut &x) // 'x' is "lent" as "mutable"
            assert(x == 7) // 'x' is still available
        }

        { // TODO: fix this
            double(n *u32) u32 { // takes the pointer to 'n'
                return n << 1 // leftward bit shift
            }
            y mut = 4 
            y = double(&y) // 'y' is consumed then recycled
            assert(y == 8)
        }

        { // implicit arguments
            add(n1 i32, n2 i32) i32 { // likely optimizes to pass-by-ref, but since thats unsure, consumes 'n1' and 'n2'
                return n1 + n2 // if optimized to pass-by-ref, implicitly uses value
            }
            h = -2; i isize = 5
            assert(add(i, h) == 3) // since 'h' and 'i' might be consumed, compiler assumes they are no longer accessable
        
        }
    }

    { // enumeration and nested structure
        Color {
            Red,
            Green,
            Blue,
            RGB {
                r: u8,
                g: u8,
                b: u8,
            }
        }

        _ = Color<RGB{ r: 0, g: 0, b:0 }> // construct the enum with nested struct
    }

    {
        hex = 0x55aa
        binary = 0b00110011
        strange = 011 
    }
}
