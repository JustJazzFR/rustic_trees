use rustic_trees::{BinaryTree, NaryTree, TraversalOrder::InOrder};

fn main() {
    //works pretty well with string types
    let mut tree1 = BinaryTree::new("Parent".to_string());
    tree1.insert(String::from("Child 1"));
    tree1.insert(String::from("Child 2"));
    println!("Size: {}", tree1.size());

    let found1 = tree1.find("Child 1".to_string());
    match found1 {
        Some(found) => unsafe {
            // "its ok rust users nothing bad will happen :)" - Wise Old C User.
            let data = (*found).data as *const String;
            if !data.is_null() {
                println!("Found: {:?}", *data);
            }
        },
        None => println!("Not found"),
    }
    println!("{:?}", tree1);

    let mut tree2 = BinaryTree::new(42);
    for i in 0..=5 {
        tree2.insert(i * i - 1);
    }
    println!("{:?}", tree2);
    let found2 = tree2.find(6); //wont be found cause there is no 6
    match found2 {
        Some(found) => unsafe {
            let data = (*found).data as *const String;
            if !data.is_null() {
                println!("Found: {:?}", *data);
            }
        },
        None => println!("Not found"),
    }
    tree2.insert(4001);
    tree2.delete(8);
    println!("{:?}", tree2);
    let collected: Vec<i32> = tree2.traverse(InOrder).filter(|n| n % 2 == 0).collect(); //iterate through the whole tree
    println!("{:?}\n", collected);
    //mutable traversal works a little differently.
    let mut transformed: Vec<i32> = vec![];
    let mut iter = tree2.traverse_mut(InOrder);
    while let Some(val) = iter.next() {
        let tval = *val * 2;
        transformed.push(tval);
    }
    println!("{:?}\n", transformed);
    //still pretty simple (i hope 🙏)

    //works with non-primitive types also
    let mut ntree1 = NaryTree::new(vec![1, 2, 3]);
    ntree1.insert(None, vec![1, 2, 3, 5, 7, 11, 13, 17]); //none is the node to insert at. in this case none inserts into the tree itself.
                                                          //you use ntree.find(node_value) to get a node to use here.
    println!("{:?}", ntree1);
    let found_vec = ntree1.find(vec![1, 2, 3]); //enter value here
    match found_vec {
        Some(nptr) => unsafe {
            let data = (*nptr).data as *const Vec<i32>;
            if !data.is_null() {
                println!("found: {:?}", *data);
            }
        },
        None => println!("Not found"),
    }
    ntree1.insert(None, vec![67, 42, 1]);
    let mut niter = ntree1.iter_mut();
    while let Some(nval) = niter.next() {
        for n in nval.iter_mut() {
            *n = 67;
        }
    }
    println!("{:?}\n", ntree1);
    ntree1.delete(None, vec![67, 67, 67]);
    println!("{:?}", ntree1);

    println!(
        "Sizes: {}, {}, {}",
        tree1.size(),
        tree2.size(),
        ntree1.size()
    );
    //nice it works
}
