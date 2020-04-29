pub extern crate wascc_actor;

// pub use wascc_actor::prelude::{
//   codec, HandlerResult,
// };

#[macro_use]
mod macros;

pub mod actor_kvp;

#[cfg(test)]
mod test {
  #[macro_use]
  use super::{actor_messaging_handlers};

  
}