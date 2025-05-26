#[macro_export]
macro_rules! subscribe {
    ($bus:expr, $event:expr, $type:ty, $handler:expr) => {
        $bus.subscribe::<$type, _>($event, $handler);
    };
}

#[macro_export]
macro_rules! emit {
    ($bus:expr, $event:expr, $target:expr, $payload:expr) => {
        $bus.emit(&$event, $target as &mut dyn std::any::Any, &$payload);
    };
}