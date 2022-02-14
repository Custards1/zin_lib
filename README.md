# Welcome
This is the zin language's home! Zin is a well equipped interpreted programming language crafted in rust, try it out today!

# Example
```
induct system;
foo => (data,func) {
    for i in iter(data){
        func(i);
    }    
}
doc => (object){
    print("Object: ",object.__doc,"\nData: ",props(object));
}
# This is a comment!
doc(system);
foo([1,2,3],$(x){print("prefixed! ",x);});
```

# Status
Currently this project is in development mode and is not ready for cargos package manager.
Breaking changes are expected in future versions.

# Installation
You must have cargo installed to continue.
Clone the repository from github:
```
$ cd zin_lib
$ cd zin
$ cargo install --path .
```

# Basics
Create a new file: 'test.zin' to get ready to program.

zin is run from the command line. To simply run your zin file at any point, open up a terminal and run:
```
$ zin test.zin
``` 
Remember, don't forget your semicolons!

Got it? Great!


# Assignments
In zin, variables may be assigned with a 'let':
```
let foo = ...;
```
or with the special ':=' syntax:
```
foo := ...;
```

# Functions

Functions may be declared with `name`=>(args..) {body...;}
```
foo =>(bar){
    #...
}
```

To return a value from a function, use the 'ret' keyword; If no value is explicitly returned, the return value will be void.
```
foo =>(bar){
    ret bar + 2;
}
```

To call a function:
```
foo(2);
```

Lambda functions may also be specified using the '$(args..){body...} syntax
```
foo := $(x){ret x+2;};
```
# Loops
There is one kind of for loop in zin, and its iterator based.
To iterate over a collection:
```
example := [1,2,3,4,5];
for i in iter(example) {
    print(i);
}
``` 
To iterate over a range of numbers:
```
for i in range(1,10) {
    print(i);
}
``` 
# Roadmap
- [ ] Bring the std to stable version
- [ ] Allow imports from other files and packages
- [ ] Interface with external C libraries
- [ ] Optimize speed and memory
- [ ] Allow async functions
- [ ] Finish object implemntation