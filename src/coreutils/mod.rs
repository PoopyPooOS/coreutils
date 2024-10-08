macro_rules! import_coreutils {
    (
        $(
            $mod_name:ident
        ),*
    ) => {
        $(
            mod $mod_name;
            pub use $mod_name::$mod_name;
        )*
    };
}

import_coreutils!(cat, clear, cp, echo, ls, ln, mkdir, mount, mv, poweroff, printenv, reboot, rm, sleep, touch);
