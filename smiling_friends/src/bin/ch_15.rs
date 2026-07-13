/* The Shared Cursed Object

**The bit:** Multiple open cases all reference the same cursed prop sitting in the
break room, and everyone keeps touching it.

**Your mission:** Model a piece of shared, mutable state referenced from more than
one place using Rc<RefCell<>>.*/

// 1. Define a struct representing the shared object (e.g. CursedProp with a name and a
//    condition field).
// 2. Wrap an instance of it in Rc<RefCell<CursedProp>>.
// 3. Clone the Rc handle and assign it to two different variables representing two
//    different cases referencing the same object.
// 4. From one "case," use .borrow_mut() to change a field on the shared object.
// 5. From the other "case," use .borrow() to read the object and confirm it sees the
//    updated value.
// 6. Print Rc::strong_count(...) at a couple of points to watch the reference count
//    change as clones are made.
// 7. (Optional) Use a plain Box<T> for something that doesn't need to be shared, just to
//    contrast it with Rc.