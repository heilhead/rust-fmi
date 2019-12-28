pub fn realign_unchecked<U, T>(data: &[U]) -> &[T] {
  unsafe { data.align_to().1 }
}

pub fn realign_unchecked_mut<U, T>(data: &mut [U]) -> &mut [T] {
  unsafe { data.align_to_mut().1 }
}
