#![feature(duration_constructors, iter_intersperse)]
#![allow(clippy::unnecessary_wraps, clippy::manual_assert)]

use coreutils_macro::coreutils;

coreutils!(
    ls,
    cat,
    clear(noargs),
    cp,
    echo,
    ln,
    mkdir,
    mount,
    mv,
    printenv(noargs),
    rm,
    sleep,
    touch
);
