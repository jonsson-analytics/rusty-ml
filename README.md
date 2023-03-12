# rusty-ml

```rusty-ml

syntax html {
  '<' $id '>' $html '</' $id '>'
} -> Html.$id ($html);


val page = with html {
  <html>
    <head> <title> Hello </title> </head>
    <body> <h1> Hello </h1> </body>
  </html>
};

val foo = `bar`;
val bar = 10;
val baz = true;
val f = fun x -> x + 1;
def g x -> x + 1;
val a = f 10;
val b = g 10;
```

## Roadmap

- implement function application in parser
- implement pretty printer
- implement REPL
- implement ML style type inference
- implement underlying CST for improved error messages
- refactor test suite (remove redundancies; maybe with pretty printer?)
- reduce use of copy
