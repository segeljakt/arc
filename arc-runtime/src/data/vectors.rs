// use crate::prelude::*;
//
// pub mod sharable {
//     use crate::prelude::*;
//
//     #[derive(Clone, From, Deref, DerefMut, Debug)]
//     #[from(forward)]
//     pub struct Vector<T: Data>(pub Gc<ConcreteVector<T>>);
//
//     pub type ConcreteVector<T> = comet::alloc::vector::Vector<T, MiniMark>;
//
//     impl<T: Data> Alloc<Vector<T>> for ConcreteVector<T> {
//         fn alloc(self, ctx: &mut Context) -> Vector<T> {
//             Vector(ctx.mutator.allocate(self, AllocationSpace::New))
//         }
//     }
//
//     unsafe impl<T: Data> Trace for Vector<T> {
//         fn trace(&mut self, vis: &mut dyn Visitor) {
//             self.0.trace(vis)
//         }
//     }
//
//     impl<T: Data> Collectable for Vector<T> {}
//     unsafe impl<T: Data> Finalize for Vector<T> {}
//     unsafe impl<T: Data> Send for Vector<T> {}
//     unsafe impl<T: Data> Sync for Vector<T> {}
//     impl<T: Data> Unpin for Vector<T> {}
// }
//
// mod sendable {
//     use crate::prelude::*;
//
//     #[derive(From)]
//     #[from(forward)]
//     pub struct Vector<T>(pub Box<[T]>);
//
//     pub type ConcreteVector<T> = std::vec::Vec<T>;
//
//     unsafe impl<T> Send for Vector<T> {}
// }
//
// impl<T: Data> IntoSendable for sharable::Vector<T> {
//     type T = sendable::Vector<T::T>;
//     fn into_sendable(self, ctx: &mut Context) -> Self::T {
//         self.0.to_vector().into()
//     }
// }
//
// impl<T: IntoSharable> IntoSharable for sendable::Vector<T>
// where
//     T::T: Data,
// {
//     type T = sharable::Vector<T::T>;
//     fn into_sharable(self, ctx: &mut Context) -> Self::T {
//         let s = Vector::with_capacity(self.0.len(), ctx);
//         s.clone().push_str(self.0.as_ref(), ctx);
//         s
//     }
// }
//
// use sharable::Vector;
