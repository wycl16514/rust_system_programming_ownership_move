For system level programming lanaguage like c/c++, memory management is totally nightmare, That's why all languages after c/c++ always bring "garbage collector" with them, and free programmer from spending their time
on memory management. There is only one truth in the universe that is you never have free launch. Garbe collector bring great performance losses, because a significant computing power will allocate to garbage 
collection.

Rust make a smart compromise for this. It builds up strict rules for programmer about how to use pointer and manage memory, and the not free launch thoery still apply, that is the grammar of rust is obviously 
difficult to grab compare with other languages. There is one principle for rust memory management, as long as we understant it, we will easily to know why Rust design such set of complated grammar rules.The root
pricinple of Rust memory management is: There always only one root owner for any allocated memory, as you can see the following image:

![rust-ownership-move](https://github.com/wycl16514/rust_system_programming_ownership_move/assets/7506958/7995a0e3-e8e8-4809-bbf5-b8098801a8db)


When we create object A, and it create its childeren B,C,D, and B, Then B creates its children E,F, and D creates its cihld G, Then the only way you can write to B,C,D,E,F,G is you are the owner of A, otherwise
you may only have the right to read to them. And except the owner of A, no one can delete B,C,D,E,F,G, when A is removed, then all its children and grand chldren are all removed.This is far different from c/c++,
in the later, any one who gets the pointer to the node in the tree can read, write and delete them, such rule is flexible but would cause caos in memory management.

Let's see an example by inspecting a piece of code, create a new project called ownership-move and add following code in main.rs:
```rs
fn print_padovan() {
    //root owner of vector is padovan
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        //create new node here as children of padovan
        padovan.push(next);
    }

    println!("len of padovan array: {}", padovan.len());
    println!("P(1..10) = {:?}", padovan)
} //padovan removed here and all its children are removed

fn main() {
    print_padovan();
}
```
When we run the code, we get the following output:
```rs
len of padovan array: 10
P(1..10) = [1, 1, 1, 2, 2, 3, 4, 5, 7, 9]
```
We can see from the code, originally we only have tree elements in the array, and we add new elements in the for loop, The variable padovan are node A in the image above, all elements in the array are children of 
padovan just like B,C,D in the image aboved, when we go to the end of function print_padovan, the node padovan is to the end of its life scope and it will be removed, this will cause all its children to be removed.

There are two kinds of memory, one is from the stack, the other is from heap, for c/c++, any memories allocate using new or alloc are from heap, in Rust the equivalence of new in c++ is Box::new, memory allocated 
from stack will removed automatically when the control run out of its scope, but memory allocated from heap will removed unless its onwer is removed, let's check an example:

```rs
fn allocated_tuple_from_heap() -> (i32, i32) {
    let tup_heap = Box::new((1, 2));
    let tup_stack = (3, 4);
    println!("tup on stack: {:?}", tup_stack);
    return *tup_heap;
} // stack_int

fn print_tup_from_head() {
    let tup_heap2 = allocated_tuple_from_heap();
    println!("vec from heap: {:?}", tup_heap)
} //memory of heap_vec drop here

fn main() {
    //print_padovan();
    print_tup_from_head();
}

```
In aboved code, tup_heap is allocated from heap, and its memory will not removed when the control go to the end of function allocated_tuple_from_heap, because the owner of its memory will transfer to tup_heap2 in
function print_tup_from_head, and the memory of tup (1, 2) will removed when control goto the end of print_tup_from_head because the root owner tup_heap2 will be removed. And notics that memory for tup_stack will 
be removed because the owner node that is tup_stack will be removed immediately when control go out of allocated_tuple_from_heap,  see the following image:

![rust-ownership-move (1)](https://github.com/wycl16514/rust_system_programming_ownership_move/assets/7506958/b7ec65e7-3d5f-4391-951a-f20e3deddad6)

The "only one root onwer" principle applied no matter how complicate the  memory allocation is, for example the following code has multiple layers of memory allocation:
```rs
fn allocate_persons() {
    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });

    for composer in &composers {
        println!("{}, born: {}", composer.name, composer.birth)
    }
}//all memory allocated for composers removed here

fn main() {
    //print_padovan();
    //print_tup_from_head();
    allocate_persons();
}

```
The memory allocation for above code is like following:



![rust-ownership-move (2)](https://github.com/wycl16514/rust_system_programming_ownership_move/assets/7506958/91911666-7738-408f-ba7a-00a03bb86f70)

We can see from the image aboved that, when we create a Person object and push it to the vector, the memory for the object is created and the content of field birth is saved in the memory, but the content of the
field name is not saved in the same memory, the name acts as a pointer in the memory, and the code allocate another chunk of memory to save the content of the field name, this results in a tree like memory 
allocation structure, but there is only one memory owner that is the root of the tree aboved, as long as the root owner is removed, all memory chunk that can travel from the root will be removed.

