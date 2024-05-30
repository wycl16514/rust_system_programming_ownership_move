fn exmaple_of_move() {
    //s is the first root owner
    let s = vec!["undon".to_string(), "ramen".to_string(), "soba".to_string()];
    //root owner transfer to t
    let t = s;
    //root owner transfter to u
    let u = t;

    //panic here, s is uninitialized
    //let v = s;

    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string(); // memory for "Govinda" will be released here
    println!("s: {}", s);
}

fn f(v: Vec<i32>) {
    println!("{:?}", v);
}

fn g(v: Vec<i32>) {
    println!("{:?}", v);
}

fn h(v: Vec<i32>) -> Vec<i32> {
    println!("{:?}", v);
    return v;
}

fn move_in_control_statements(c: i32) {
    let x = vec![10, 20, 30];
    if c > 0 {
        f(x); //ok to move ownership of x into function f
    } else {
        g(x); //ok to move ownership of x into function g
    }
    //h(x); //bad here, because x is moved to f or g already and becauses unintialized
}

fn move_vec_elemets() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    //let third = v[2]; //bad, v[2] has its root owner as v
    //let fifth = v[4]; //bad, v[4] has its root owner as v
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

struct Person {
    name: Option<String>,
    birth: i32,
}

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

use std::rc::Rc;

fn reference_count() {
    // let s = vec!["hello".to_string(), "world".to_string()];
    // let t = s.clone();
    // let u = s; //ok, because s is still own the vector

    /*
    clone will not make any deep copy just increate the reference count
    */
    let s = Rc::new(vec!["hello".to_string(), "world".to_string()]);
    let t = s.clone();
    let u = s.clone();
    //notice reference count is immuatable
    s[0].push('!');
}

fn main() {
    reference_count();
}
