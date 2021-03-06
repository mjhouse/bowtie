macro_rules! flash {
    ( $p:expr, $m:expr ) => { Err(Flash::error(Redirect::to($p), $m)) }
}

macro_rules! unflash {
    ( $f:expr ) => { 
        $f.map(|msg| Some(msg.msg().to_string()))
          .unwrap_or_else(|| None)
    }
}

macro_rules! context {
    ( $( $k:expr => $v:expr ),* ) => {
        {
            let mut c = Context::new();
            $( c.insert($k,&$v); )*
            c
        }
    }
}