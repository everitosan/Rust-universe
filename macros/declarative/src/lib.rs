#[macro_export]
macro_rules! hello {
  () => {
    println!("Hola mundo");
  };
}

#[macro_export]
macro_rules! map {
    ($key: ty, $val:ty) => {
        {
          let map: HashMap<$key, $val> = HashMap::new();
          map
        }
    };
    ($($key: expr => $val: expr),*) => {
      {
        let mut map = HashMap::new();
        $( map.insert($key, $val); )*
        map
      }
    }
}