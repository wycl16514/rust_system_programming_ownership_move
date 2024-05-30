In last section, we know root owner in memory management of Rust, when we move the ownership to new one, the oringinal root owner becomes unintialized, if we assign the old root owner to
other variable, Rust compiler will report error and adivce you using clone, for example we have the following code:
```rs
fn reference_count() {
    let s = vec!["hello", "world"];
    let t = s;
    let u = s;
}

fn main() {
    reference_count();
}
```
When we run the aboved code, we will get the following error:

<img width="1048" alt="截屏2024-05-28 23 03 01" src="https://github.com/wycl16514/rust_system_programming_ownership_move/assets/7506958/c4b484eb-5ca8-4f44-a3b0-df106a0770f5">

In order to prevent s becomes uninitialized, we can use clone to make a deep copy, if we change the code like following:

```rs
fn reference_count() {
    let s = vec!["hello", "world"];
    //let t = s;
    //make deep copy
    let t = s.clone();
    let u = s;
}
```
Then the memory model will like following:

![rust_move](https://github.com/wycl16514/rust_system_programming_ownership_move/assets/7506958/b16a7e0f-96e2-4735-b27d-9da6380908b4)

As we can see, the clone method make a deep copy for the original data, and now we have two copy of string vectors. Now we can change data belongs to root owner t without effecting root owner u. 

The problem for this method is, when the original memory structure is complex or huge, the time need to make the deep copy is long and we need to wast another chunk of memory to save the same data, if we only need to read into the data without chaning it, such method will cause serious 
performance loss.

In order to avoid any unneccesary memory deep copy and the performace demage, Rust provide mechanism for memory sharing by using reference count. 
Which means a chunk of memory can be "owned" by multiple root owners and there is a reference count to remember how many owners are owing the data
chunk, when one owner go out of its scope and invalided, the reference count is reduce by 1, and if the reference count reduce to 0, the memory chunk
will be released.

Let's see a code example:
```rs

```
