# Rage Directives

## About Directives
A better form of macros.

Once a directives are assumed exported and are therefore publicly available once their module is imported.

## Built-in Directives

Directives that come baked into the bootstrapper.

- `#run` : Executes the following code like a script. Just-In-Time (JIT) compiled.
- `#use` : Pulls in a another module.
- `#define` : Define a directive from the following code.
- `#if` : Conditional code inclusion.

## Standard Directives

Directives that come default with the language.

- `#test` : Executes the following code and reports a success or error status.
- `#build` : Create an executable from the following code. Ahead-Of-Time (AOT) compiled.
- `#import` : Pull in another source.
- `#[attribute1, attribute2, ...]` : Declare a list of attributes for the following code.

## Examples
Insert macro into code.
```rage 
#define { echo $(s string) } = {
    println($s)
}

#define MESSAGE = "Hi Mom!" // same as #define {MESSAGE} = {"Hi Mom!"}

#run {
    echo MESSAGE // same as println("Hi Mom!")
}
```
