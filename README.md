# SLR Compiler with Axum and Tera

## About
this repository hosts an SLR (Simple LR parser) compiler project made for me to learn the basics of it in university,
utilizing Axum for the web framework and Tera as the templating language.
the compiler focuses on lexical token analysis and syntax checking for a simplified programming language.

## Features
- **Lexical Analysis:** Tokenizes the input code, identifying keywords, operators, literals, and identifiers.
- **Syntax Checking:** Utilizes SLR parsing techniques to validate the structure of the code against grammar rules.

## Example Syntax
The compiler supports basic conditional and assignment operations:

```
fn main() {
   if (x <= 10) {
      x = 10 + 32;
   };
}
```
