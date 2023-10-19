#[macro_export]
macro_rules! singleton {
    ($name:ident: $ty:ty = $expr:expr) => {
        $crate::_export::critical_section::with(|_| {
            static mut $name: (::core::mem::MaybeUninit<$ty>, bool) =
                (::core::mem::MaybeUninit::uninit(), false);
            
            #[allow(unsafe_code)]
            let used = unsafe { $name.1 };
            if used {
                None
            } else {
                let expr = $expr;

                #[allow(unsafe_code)]
                unsafe {
                    $name.1 = true;
                    $name.0 = ::core::mem::MaybeUninit::new(expr);
                    Some(&mut *$name.0.as_mut_ptr())
                }
            }
        })
    };
    (: $ty:ty = $expr:expr) => {
        $crate::singleton!(VAR: $ty = $expr)
    };
}