As we have seen in previous section, for every memory chunk that is allocated from stack or heap, we can track back its root owner, you can only write
to or release that memory chunk unless you have the ownership of the root owner. In order to gain more flexible for the language, Rust allow to move
the ownership of the root owner, for following cdoe example:
```rs
//s is the first root owner
let s = vec!["undon".to_string(), "ramen".to_string(), "soba".to_string()]
//root owner transfer to t
let t = s;
//root owner transfter to u
let u = t;

//panic here, s is uninitialized
let v = s;
```
The root owner is moved to new one when the assignment is executed as its shown by following image:

![rust_move](https://github.com/wycl16514/rust_system_programming_ownership_move/assets/7506958/c0874151-6bd0-4252-914f-29469efbe47a)

As we can see from the image aboved, .to_string() convert each string literal to struct String, and there are three fields in String namely ptr, cap,
and len, ptr point to the memory chunk where the string characters are saved, cap indicates the length of the buff used to save the string characters
,and len used to indicate the length of the string.

Originally the first root owner is variable s, when the assign statement let t=s; is excuted, the root owner transfter to t, and s is leaved in an 
uninitialized state, if we have any operation on s, panic will happen. When the second assig statement is executed, the root onwer transfter from t to u.

When we assign new value to an already initialized variable, the original assigned value will be release, as the following code:
```rs
let mut s = "Govinda".to_string();
s = "Siddhartha".to_string();// memory for "Govinda" will be released here
```

The memory management will like the following image:

![rust_move (1)](https://github.com/wycl16514/rust_system_programming_ownership_move/assets/7506958/56e51030-89be-4130-ab43-e5bd9ec80c35)

but the following code will cause not memory to dropp:
```rs
let mut s = "Govinda".to_string();
let t = s;
s =  "Siddhartha".to_string();
```
The transfer of ownership will like following:

![rust_move (2)](https://github.com/wycl16514/rust_system_programming_ownership_move/assets/7506958/079fec29-dc94-4ad7-a3bb-86c1708be6b6)

The ownership move no only happen in assigment, but also in passing value to paremeters of functions, or returning value from function. The move of
ownership is a little tricky in control statements for example:
```rs
fn f(v :Vec<i32>) {
    println!("{:?}", v);
}

fn g(v :Vec<i32>) {
    println!("{:?}", v);
}

fn h(v :Vec<i32>) -> Vec<i32> {
    println!("{:?}", v);
    return v;
}

fn move_in_control_statements(c int) {
    let x = vec![10, 20, 30];
    if c > 0 {
        f(x); //ok to move ownership of x into function f
    } else {
        g(x); //ok to move ownership of x into function g
    }
    h(x); //bad here, because x is moved to f or g already and becauses unintialized
}

```
And we need to make sure the ownership of the original owner can't move multiple times in loop as following:
```rs
fn cannot_move_in_loop() {
    let mut count = 3;
    let x = vec![10, 20, 30]
     while count > 0 {
        f(x); // bad, x is invalid after the first loop
    }
}
```
but the following is allow:
```rs
fn can_move_in_loop() {
    let mut count = 3;
    let x = vec![10, 20, 30]
     while count > 0 {
        x = h(x); //the ownership move to h and move back to x again 
    }

    g(x);
}
```
If an object has root owner, we can't simply move it to other object, and we need given methods or rules to indicate the roo owner changes, such as :
```rs
fn move_vec_elemets() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    let third = v[2]; //bad, v[2] has its root owner as v
    let fifth = v[4]; //bad, v[4] has its root owner as v
    /*
    if an object has root owner, we need to remove the root owner before move its ownsip to new one
    */
    //pop remove the last element from its root owner and the element becomes its own root owner
    // the assigment make a new root owner to the last element
    let fifth = v.pop().expect("vector empty");
    println!("fifth: {}", fifth);
    /*
    swap_remove(index) remove the element in the given place from its root owner and move the last element to the given place
    */
    let second = v.swap_remove(1);
    println!("second is :{}", second);

    /*
    std::mem::replace() replace the string content for the given String object
    */
    let third = std::mem::replace(&mut v[2], "substitue".to_string());
    println!("third is :{}", third);
    println!("v[2] is :{}", v[2]);

      /*
    for loop remove every element in v and v is no longer root owner for them
    */
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
    //bad, v contains no elment
    //println!("v :{:?}", v);
}
```

When an object has a root owner other than itself, we can't simply reassign it to new root owner we need to use some method to do it, there are
a set of methods for String object as following:
```rs
fn change_owner() {
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Palestrina".to_string()),
        birth: 11525,
    });
    /*
    composers[0].name has root owner that is composer and can't change its root owner
    by simply assignment
    */
    //let first_name = composers[0].name;

    /*
    std::mem::replace remove the object from its root owner and create a new elment and
    assign the root owner to the new element
    */
    let first_name = std::mem::replace(&mut composers[0].name, Some("new name".to_string()));
    println!("first name: {:?}", first_name);
    //take has same effect above, but the new value set to None
    let second_name = composers[0].name.take();
    println!("second name: {:?}", second_name);
    println!("composers[0].name: {:?}", composers[0].name);
}
```

