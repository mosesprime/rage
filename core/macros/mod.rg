//*
* Rage Core Macros
*/

/// Prints a new line to `stdout`.
#define println(...$arg ToString) = {
    io.get_stdout().write($arg).flush("\n")
}
