#[macro_export]
macro_rules! __impl_try_collect_tuple {
  () => { };
  ($A:ident $($I:ident)*) => {
      __impl_try_collect_tuple!($($I)*);

      impl<$A: Iterator> TryCollect<($A::Item, $($I::Item),*)> for $A {
          fn try_collect(&mut self) -> Option<($A::Item, $($I::Item),*)> {
              let r = (__try_opt!(self.next()),
                      // hack: we need to use $I in the expasion
                      $({ let a: $I::Item = __try_opt!(self.next()); a}),* );
              Some(r)
          }
      }
  }
}

#[macro_export]
macro_rules! __try_opt {
  ($e:expr) => (match $e { Some(e) => e, None => return None })
}

#[macro_export]
macro_rules! __try_collect {
  () => {
    trait TryCollect<T> {
      fn try_collect(&mut self) -> Option<T>;
    }
    
    // implement TryCollect<T> where T is a tuple with size 1, 2, .., 10
    __impl_try_collect_tuple!(A A A A A A A A A A);
  }
}

#[macro_export]
macro_rules! actor_messaging_handlers {
  ( $( $key:pat => $handler:expr ),* $(,)? ) => {

    // use $crate::wascc_actor::prelude::{actor_handlers};

    __try_collect!();



    fn handle_nats_message(msg: BrokerMessage) -> HandlerResult<()> {
      println(&format!("Received broker message: {:?}", msg));
      let mut fillup_to_ten: Vec<&str> = msg.subject.split('.').collect() ;
      loop{ 
        if fillup_to_ten.len() >= 10 {break};
        fillup_to_ten.push("");
      }
      let splited: (&str, &str, &str, &str, &str, &str, &str, &str, &str, &str) = fillup_to_ten.into_iter().try_collect().unwrap();
      println(&format!("splited: {:?}", &splited));

      match splited {
        $(
          $key => { $handler(&msg); },
        )*

        _=>{
          println(&format!("Unhandled broker message: {:?}", msg));
        },
      }

      Ok(())
    }
  }
}