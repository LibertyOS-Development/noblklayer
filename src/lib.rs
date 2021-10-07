#![no_std]
use core::fmt;

pub type Result<T, E> = ::core::result::Result<T, Err<E>>;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Err<E>
{
	Other(E),
	WouldBlk,
}
impl<E> fmt::Debug for Err<E>
where
	E: fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		match *self
		{
			Err::Other(ref e) => fmt::Debug::fmt(e, f),
			Err::WouldBlk => f.write_str("WouldBlk"),
		}
	}
}
impl<E> Err<E>
{
	pub fn map<T, F>(self, op: F) -> Err<T>
	where
		F: FnOnce(E) -> T,
	{
		match self
		{
			Err::Other(e) => Err::Other(op(e)),
			Err::WouldBlk => Err::WouldBlk,
		}
	}
}
impl<E> From<E> for Err<E>
{
	fn from(err: E) -> Err<E>
	{
		Err::Other(err)
	}
}
#[macro_export]
macro_rules! blk
{
	($e:expr) =>
	{
		loop
		{
			#[allow(unreachable_patterns)]
			match $e
			{
				Err($crate::Err::Other(e)) =>
				{
					#[allow(unreachable_code)]
					break Err(e)
				}
				Err($crate::Err::WouldBlk) => {}
				Ok(x) => break Ok(x),
			}
		}
	};
}
