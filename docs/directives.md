# Rage Directives

## About Directives
A better form of macros.

## Built-in Directives

Directives that come baked into the bootstrapper.

- `#run` : Executes the following code like a script. Just-In-Time (JIT) compiled.
- `#use` : Pulls in a another module.
- `#define` : Define a directive from the following code.

## Standard Directives

Directives that come default with the language.

- `#test` : Executes the following code and reports a success or error status.
- `#build` : Create an executable from the following code. Ahead-Of-Time (AOT) compiled.
- `#import` : Pull in another source.
- `#[attribute1, attribute2]` : Declare a list of attributes for the following code.
