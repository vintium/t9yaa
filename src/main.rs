
use t9yaa::Layout;

fn main() {
    let st9 = Layout::standard_t9();
    dbg!((st9.key_for('c'), st9.values_of('2')));
}
