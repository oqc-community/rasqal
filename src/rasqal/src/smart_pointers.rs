// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2024 Oxford Quantum Circuits Ltd

use std::cell::Cell;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, BitAnd, BitOr, BitXor, Deref, DerefMut, Div, Index, Mul, Rem, Shl, Shr, Sub};
use std::panic::UnwindSafe;

pub type Ptr<T> = FlexiPtr<T>;

/// Allows an expression call to be done mutably without resorting to borrow casting. Use this
/// when you need to override the mutability constraint as it's the safest version, ironically.
#[macro_export]
macro_rules! with_mutable {
    ($val:ident.$($rest:tt)*) => {
        unsafe {
            (*$val.as_ptr()).$($rest)*
        }
    };
}

/// See [`with_mutable`]. Just a modification of that macro that allows the target pointer
/// to be a field on an object (since you can't seemingly match on 'self').
#[macro_export]
macro_rules! with_mutable_self {
    ($self:ident.$val:ident$($rest:tt)*) => {
        unsafe {
            (*$self.$val.as_ptr())$($rest)*
        }
    };
}

/// `FlexiPtr`
/// current live references.
pub struct FlexiRef<T: ?Sized> {
  counter: *mut Cell<usize>,
  value: *mut Cell<T>
}

impl<T: ?Sized> Drop for FlexiRef<T> {
  fn drop(&mut self) {
    unsafe {
      drop(Box::from_raw(self.value));
      drop(Box::from_raw(self.counter));
    }
  }
}

impl<T> FlexiRef<T> {
  pub fn new(value: T, initial_count: usize) -> FlexiRef<T> {
    FlexiRef {
      value: Box::into_raw(Box::new(Cell::new(value))),
      counter: Box::into_raw(Box::new(Cell::new(initial_count)))
    }
  }
}

impl<T: ?Sized> FlexiRef<T> {
  /// Increases ref-count by 1.
  pub fn inc(&self) {
    unsafe {
      let counter = self.counter.as_ref().unwrap();
      counter.set(counter.get() + 1);
    }
  }

  /// Increases ref-count by argument.
  pub fn inc_by(&self, val: usize) {
    unsafe {
      let counter = self.counter.as_ref().unwrap();
      counter.set(counter.get() + val);
    }
  }

  /// Decreases ref-count by 1.
  pub fn dec(&mut self) {
    unsafe {
      let counter = self.counter.as_ref().unwrap();
      counter.set(counter.get() - 1);
    }
  }

  /// Decreases ref-count by argument.
  pub fn dec_by(&mut self, val: usize) {
    unsafe {
      let counter = self.counter.as_ref().unwrap();
      counter.set(counter.get() - val);
    }
  }

  /// Returns the count of currently active references this object has.
  pub fn ref_count(&self) -> usize { unsafe { (*self.counter).get() } }

  /// Fetch the inner pointer as a reference.
  pub fn value(&self) -> &mut T { unsafe { (*self.value).get_mut() } }
}

/// Cloning a reference means just copying across the pointers.
impl<T: ?Sized> Clone for FlexiRef<T> {
  fn clone(&self) -> Self {
    FlexiRef {
      value: self.value,
      counter: self.counter
    }
  }
}

impl<T: ?Sized> PartialEq for FlexiRef<T> {
  fn eq(&self, other: &Self) -> bool {
    self.counter.addr() == other.counter.addr() && self.value.addr() == other.value.addr()
  }
}

impl<T: ?Sized> Eq for FlexiRef<T> {}

impl<T: Display> Display for FlexiRef<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(
      format!(
        "Value: [{}] '{}', count: [{}] {}",
        self.value.addr(),
        self.value(),
        self.counter.addr(),
        self.ref_count()
      )
      .as_str()
    )
  }
}

/// Reference-counted smart-pointer which avoids Rusts mutation and lifetime rules.
///
/// Functionally this was built to act as a C++-like smart pointer leaving lifetime and
/// potential dangerous usages up to the writer, not Rusts various analysis'.
///
/// It acts as a super-pointer, merging owned objects and borrowed references into one structure
/// and treating them as (mostly) the same. There are some operations which cannot be performed
/// on pointers of differing types due to the structure of the internal data.
///
/// Since its internals are raw pointers Rusts lifetime rules have no clue about them, and since
/// raw pointers are also treated specially in regards to the mutation you can take out
/// infinite mutable aliases if performed through the raw pointer itself.
///
/// Due to this constraint around mutation, we use macros that perform the operations on
/// the raw pointers instead of returning them as borrows - which violates Rusts rules in certain
/// situations, multi-mutable-aliasing being one of them.
pub enum FlexiPtr<T: ?Sized> {
  None,
  RefCounted(*mut FlexiRef<T>),
  Borrow(*mut T)
}

// The flexi-pointer expects the writer to know what they're doing so sinkhole various
// annotations which are unsafe only when you don't use it properly.
unsafe impl<T: ?Sized> Send for FlexiPtr<T> {}
unsafe impl<T: ?Sized> Sync for FlexiPtr<T> {}
impl<T> UnwindSafe for FlexiPtr<T> {}

impl<T: ?Sized + Display> Display for FlexiPtr<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if !FlexiPtr::is_null(self) {
      self.deref().fmt(f)
    } else {
      Ok(())
    }
  }
}

impl<T> FlexiPtr<T> {
  /// Replaces the inner pointer with the passed-in one.
  ///
  /// This has a cascading effect so that if there are 5 smart-pointers all pointing at a
  /// single object, and you replace that inner object, all those 5 pointers will then now point to
  /// the *new* object.
  pub fn expand_into(&self, val: &FlexiPtr<T>) {
    match self {
      FlexiPtr::RefCounted(ref_) => {
        match val {
          FlexiPtr::RefCounted(other_ref) => {
            unsafe {
              // We don't want to merge the same pointers into each other.
              let is_the_same = (**ref_) == (**other_ref);
              if is_the_same {
                return;
              }

              let old_count = (**ref_).ref_count();

              // Assigning flexi-ref to the structure overwrites the pointers value
              // for that struct only, not every one. So need to just assign the
              // inner pointers instead.
              (**ref_).value = (**other_ref).value;
              (**ref_).counter = (**other_ref).counter;

              (**ref_).inc_by(old_count);
            }
          }
          FlexiPtr::Borrow(_) => {
            panic!("Can't extract from ref-counted into borrow-driven flexi-pointer.");
          }
          _ => {}
        }
      }
      FlexiPtr::Borrow(borrow) => {
        match val {
          FlexiPtr::RefCounted(_) => {
            panic!("Can't extract from ref-counted into borrow-driven flexi-pointer.");
          }
          FlexiPtr::Borrow(other_borrow) => {
            // TODO: Probably doesn't work, test and fix.
            // Reading a reference with a bitwise copy is probably fine as it's just a
            // pointer anyway.
            unsafe { (*borrow).write(other_borrow.read()) };
          }
          _ => {}
        }
      }
      _ => {}
    }
  }
}

impl<T: ?Sized> FlexiPtr<T> {
  pub fn is_null(self_: &Self) -> bool {
    match self_ {
      FlexiPtr::None => true,
      _ => false
    }
  }

  pub fn is_not_null(self_: &Self) -> bool { !FlexiPtr::is_null(self_) }

  /// Returns the address of the inner object.
  pub fn as_address(self_: &Self) -> usize {
    // TODO: There's got to be a better way to get the address than this.
    self_.deref() as *const T as *mut T as *mut () as usize
  }

  /// Checks equality against pointers.
  pub fn eq(self_: &Self, other: &Self) -> bool {
    if FlexiPtr::is_null(self_) && FlexiPtr::is_null(other) {
      true
    } else if FlexiPtr::is_null(self_) || FlexiPtr::is_null(other) {
      false
    } else {
      FlexiPtr::as_address(self_) == FlexiPtr::as_address(other)
    }
  }

  /// Returns the internal value as a pointer. Panics if it's None.
  pub fn as_ptr(&self) -> *mut T {
    match self {
      FlexiPtr::RefCounted(val) => unsafe { (*(**val).value).as_ptr() },
      FlexiPtr::Borrow(val) => *val,
      _ => panic!("Attempted deref on null pointer.")
    }
  }

  /// Returns count of currently live pointers in this network of flexi-pointers.
  /// Will return None for things that don't have a ref-count.
  fn ref_count(&self) -> Option<usize> {
    match self {
      FlexiPtr::RefCounted(ref_) => unsafe { Some((**ref_).ref_count()) },
      _ => None
    }
  }

  /// We want to be able to drop the internals outside of the usual drop
  /// pathways, so have a seperate method.
  fn drop_internals(&mut self) {
    unsafe {
      // Borrows don't require a drop, neither does None, only if we're the owner of
      // an object do we want to do anything to it.

      if let FlexiPtr::RefCounted(ref_) = self {
        (**ref_).dec();
        if (**ref_).ref_count() <= 0 {
          drop(Box::from_raw(*ref_));
        }
      }
    }
  }
}

impl<T: ?Sized + Clone> FlexiPtr<T> {
  /// Clones the inner object and returns a new pointer from it.
  pub fn clone_inner(&self) -> FlexiPtr<T> { Ptr::from(self.deref().clone()) }
}

impl<T: ?Sized> Default for FlexiPtr<T> {
  fn default() -> Self { FlexiPtr::None }
}

impl<T: ?Sized> Drop for FlexiPtr<T> {
  fn drop(&mut self) { self.drop_internals() }
}

impl<T: ?Sized> Deref for FlexiPtr<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    match self {
      FlexiPtr::RefCounted(val) => unsafe { (*(*val)).value() },
      FlexiPtr::Borrow(val) => unsafe { val.as_ref().unwrap() },
      _ => panic!("Attempted deref on null pointer.")
    }
  }
}

impl<T: ?Sized> DerefMut for FlexiPtr<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    match self {
      FlexiPtr::RefCounted(val) => unsafe { (*(*val)).value() },
      FlexiPtr::Borrow(val) => unsafe { val.as_mut().unwrap() },
      _ => panic!("Attempted deref on null pointer.")
    }
  }
}

impl<T: ?Sized + PartialEq> PartialEq<Self> for FlexiPtr<T> {
  fn eq(&self, other: &Self) -> bool {
    if FlexiPtr::is_null(self) && FlexiPtr::is_null(other) {
      true
    } else if FlexiPtr::is_null(self) || FlexiPtr::is_null(other) {
      false
    } else {
      self.deref() == other.deref()
    }
  }
}

impl<T: ?Sized + PartialEq> Eq for FlexiPtr<T> {}

impl<T: Hash> Hash for FlexiPtr<T> {
  fn hash<H: Hasher>(&self, state: &mut H) { self.deref().hash(state) }
}

impl<T: ?Sized> Clone for FlexiPtr<T> {
  /// Clones the outer object, leaving the inner object pointing at the same thing.
  fn clone(&self) -> Self {
    match self {
      FlexiPtr::RefCounted(val) => unsafe {
        (**val).inc();
        FlexiPtr::RefCounted(val.clone())
      },
      FlexiPtr::Borrow(val) => FlexiPtr::Borrow(*val),
      FlexiPtr::None => FlexiPtr::None
    }
  }
}

impl<T> From<&FlexiPtr<T>> for FlexiPtr<T> {
  fn from(value: &FlexiPtr<T>) -> Self { value.clone() }
}

impl<T> From<T> for FlexiPtr<T> {
  fn from(value: T) -> Self {
    FlexiPtr::RefCounted(Box::into_raw(Box::new(FlexiRef::new(value, 1))))
  }
}

/// Steals the reference and disassociates it from lifetime tracking. Only use
/// this in situations where other things enforce that the reference won't be dropped before
/// this falls out of use.
impl<T: ?Sized> From<&T> for FlexiPtr<T> {
  fn from(value: &T) -> Self { FlexiPtr::Borrow(value as *const T as *mut T) }
}

/// Disassociates reference from lifetime tracking. See immutable declaration for more details.
impl<T: ?Sized> From<&mut T> for FlexiPtr<T> {
  fn from(value: &mut T) -> Self { FlexiPtr::Borrow(value) }
}

impl<T: Add> Add for FlexiPtr<T> where for<'a> &'a T: Add<&'a T, Output = T> {
  type Output = T;

  fn add(self, rhs: Self) -> Self::Output {
    self.deref() + rhs.deref()
  }
}


impl<T: Sub> Sub for FlexiPtr<T> where for<'a> &'a T: Sub<&'a T, Output = T> {
  type Output = T;

  fn sub(self, rhs: Self) -> Self::Output {
    self.deref() - rhs.deref()
  }
}

impl<T: Mul> Mul for FlexiPtr<T> where for<'a> &'a T: Mul<&'a T, Output = T> {
  type Output = T;

  fn mul(self, rhs: Self) -> Self::Output {
    self.deref() * rhs.deref()
  }
}

impl<T: Div> Div for FlexiPtr<T> where for<'a> &'a T: Div<&'a T, Output = T> {
  type Output = T;

  fn div(self, rhs: Self) -> Self::Output {
    self.deref() / rhs.deref()
  }
}

impl<T: BitOr> BitOr for FlexiPtr<T> where for<'a> &'a T: BitOr<&'a T, Output = T> {
  type Output = T;

  fn bitor(self, rhs: Self) -> Self::Output {
    self.deref() | rhs.deref()
  }
}

impl<T: BitAnd> BitAnd for FlexiPtr<T> where for<'a> &'a T: BitAnd<&'a T, Output = T> {
  type Output = T;

  fn bitand(self, rhs: Self) -> Self::Output {
    self.deref() & rhs.deref()
  }
}

impl<T: BitXor> BitXor for FlexiPtr<T> where for<'a> &'a T: BitXor<&'a T, Output = T> {
  type Output = T;

  fn bitxor(self, rhs: Self) -> Self::Output {
    self.deref() ^ rhs.deref()
  }
}

impl <T: Shl> Shl for FlexiPtr<T> where for<'a> &'a T: Shl<&'a T, Output = T> {
  type Output = T;

  fn shl(self, rhs: Self) -> Self::Output {
    self.deref() << rhs.deref()
  }
}

impl<T: Shr> Shr for FlexiPtr<T> where for<'a> &'a T: Shr<&'a T, Output = T> {
  type Output = T;

  fn shr(self, rhs: Self) -> Self::Output {
    self.deref() >> rhs.deref()
  }
}

impl<T: Rem> Rem for FlexiPtr<T> where for<'a> &'a T: Rem<&'a T, Output = T> {
  type Output = T;

  fn rem(self, rhs: Self) -> Self::Output {
    self.deref() % rhs.deref()
  }
}

impl<T, A: Index<A>> Index<A> for FlexiPtr<T> where T: Index<A> {
  type Output = <T as Index<A>>::Output;

  fn index(&self, index: A) -> &Self::Output {
    &self.deref()[index]
  }
}

#[cfg(test)]
mod tests {
  use crate::smart_pointers::FlexiPtr;
  use std::assert_eq;
  use std::borrow::Borrow;
  use std::fmt::{Display, Formatter};

  struct Recursive {
    nested_flexi: FlexiPtr<Recursive>,
    value: String
  }

  impl Display for Recursive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.write_str(self.value.as_str()) }
  }

  #[test]
  fn nested_smart_pointers() {
    unsafe {
      let mut starter = FlexiPtr::from(Recursive {
        nested_flexi: FlexiPtr::default(),
        value: String::from("Hello, can I have some cake?")
      });

      starter.nested_flexi = starter.clone();

      unsafe fn nested_calls(arg: &FlexiPtr<Recursive>) {
        let mut new_pointer = arg.clone();
        unsafe fn further_nested_calls(arg: &FlexiPtr<Recursive>) {
          let mut another_pointer = arg.clone();
          assert_eq!(another_pointer.ref_count().expect("Has to be active."), 4);
          assert_eq!(
            FlexiPtr::as_address(arg),
            FlexiPtr::as_address(&another_pointer)
          );
          another_pointer.value = String::from("I am no longer asking.");
        }
        further_nested_calls(new_pointer.borrow());
        assert_eq!(new_pointer.value, String::from("I am no longer asking."));
        new_pointer.value = String::from("That cake now belongs to me.");
        assert_eq!(new_pointer.ref_count().expect("Has to be active."), 3);
      }
      nested_calls(starter.borrow());
      assert_eq!(starter.value, String::from("That cake now belongs to me."));
      assert_eq!(starter.ref_count().expect("Has to be active."), 2);
    }
  }

  #[test]
  fn replace_test() {
    let starter = FlexiPtr::from(5);
    let second = starter.clone();
    let third = second.clone();
    assert_eq!(third.ref_count().expect("Exists."), 3);

    let replacement = FlexiPtr::from(10);
    starter.expand_into(replacement.borrow());

    assert_eq!(starter.ref_count().expect("Exists."), 4);
    assert_eq!(*starter, *replacement);
    assert_eq!(*starter, *second);
    assert_eq!(*second, *third);
  }

  #[test]
  fn recursive_replace() {
    let starter = FlexiPtr::from(Recursive {
      nested_flexi: Default::default(),
      value: "Dave".to_string()
    });
    let second = FlexiPtr::from(Recursive {
      nested_flexi: Default::default(),
      value: "Dave".to_string()
    });

    let first_rc = starter.ref_count().unwrap();
    starter.expand_into(&second);
    let second_rc = starter.ref_count().unwrap();

    for _ in 0..40 {
      starter.expand_into(&second);
    }

    let final_rc = starter.ref_count().unwrap();

    assert_eq!(final_rc, 2);
  }

  #[test]
  fn complicated_expansion() {
    let starter = FlexiPtr::from(Recursive {
      nested_flexi: Default::default(),
      value: "Dave".to_string()
    });
    let mut slist = Vec::new();

    // Copy to make sure clone propagates pointers.
    for _ in 0..40 {
      slist.push(starter.clone());
    }

    let second = FlexiPtr::from(Recursive {
      nested_flexi: Default::default(),
      value: "Dave the second".to_string()
    });
    let mut dlist = Vec::new();

    for _ in 0..40 {
      dlist.push(second.clone());
    }

    // Only replace half of our objects. Since everything is linked, they should all
    // change. This enforces that this link isn't broken.
    for i in 0..20 {
      slist
        .get_mut(i)
        .unwrap()
        .expand_into(&dlist.get_mut(i).unwrap());
    }

    // Iterate through everything, they should now all be the same value and ref count.
    for val in slist.iter() {
      assert_eq!(val.value, second.value);
      assert_eq!(
        val.ref_count().expect("Should be a reference"),
        second.ref_count().expect("Should be a reference")
      );
    }

    for val in dlist.iter() {
      assert_eq!(val.value, second.value);
      assert_eq!(
        val.ref_count().expect("Should be a reference"),
        second.ref_count().expect("Should be a reference")
      );
    }

    assert_eq!(starter.value, "Dave the second".to_string());
    assert_eq!(starter.ref_count().unwrap(), 82);
  }
}
