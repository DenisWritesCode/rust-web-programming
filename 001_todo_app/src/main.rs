mod to_do;
use to_do::to_do_factory;
use to_do::enums::TaskStatus;
use to_do::ItemTypes;

fn main() {
    let to_do_item = to_do_factory("washing", TaskStatus::DONE);
    match to_do_item {
        ItemTypes::Done(item) => {
            println!("{}", item.super_struct.status.stringify());
            println!("{}", item.super_struct.title);
        },
        ItemTypes::Pending(item) => {
            println!("{}", item.super_struct.status.stringify());
            println!("{}", item.super_struct.title);
        }
    }
    // let done = Done::new("shopping");
    // println!("{}", done.super_struct.title);
    // println!("{}", done.super_struct.status.stringify());

    // let pending = Pending::new("laundry");
    // println!("{}", pending.super_struct.title);
    // println!("{}", pending.super_struct.status.stringify());
}
