use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64,
}

fn main() {
    let base: Rc<RefCell<GroundStation>> =
        Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));
    println!("{:?}", base);

    {
        let mut base2 = base.borrow_mut();
        base2.radio_freq -= 12.34;
        println!("base2: {:?}", base2);
    }

    println!("base: {:?}", base);

    let mut base3 = base.borrow_mut();
    base3.radio_freq += 43.21;

    println!("base: {:?}", base);
    println!("base3: {:?}", base3);
}
