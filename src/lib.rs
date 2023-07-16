sync_stream_struct_proc::sync_stream_struct_proc!();

#[macro_export]
macro_rules! sync_stream {
    () => {
        panic!("No streams to sync");
    };
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        $crate::SyncStream2::new($a, $b)
    };
    ($a:expr, $b:expr, $c:expr) => {
        $crate::SyncStream3::new($a, $b, $c)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::SyncStream4::new($a, $b, $c, $d)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        $crate::SyncStream5::new($a, $b, $c, $d, $e)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
        $crate::SyncStream6::new($a, $b, $c, $d, $e, $f)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr) => {
        $crate::SyncStream7::new($a, $b, $c, $d, $e, $f, $g)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr) => {
        $crate::SyncStream8::new($a, $b, $c, $d, $e, $f, $g, $h)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr) => {
        $crate::SyncStream9::new($a, $b, $c, $d, $e, $f, $g, $h, $i)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr) => {
        $crate::SyncStream10::new($a, $b, $c, $d, $e, $f, $g, $h, $i, $j)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr) => {
        $crate::SyncStream11::new($a, $b, $c, $d, $e, $f, $g, $h, $i, $j, $k)
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $j:expr, $k:expr, $l:expr) => {
        $crate::SyncStream12::new($a, $b, $c, $d, $e, $f, $g, $h, $i, $j, $k, $l)
    };
}
