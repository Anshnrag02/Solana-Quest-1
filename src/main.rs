//
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let x = (5,"pokemon",false);// mut is reqd. otherwise variables are immutable
    println!("{}, world! {}",x.1,x.2);
}

