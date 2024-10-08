single call binaries: 7.6M (all musl static)
multi call binary:
 - 848K musl static
 - 748K glibc dynamic

## ls
- make better use of horizontal space:
```
this ls:2
build  cat  clear  coreutils  coreutils.d  cp  deps  echo  examples  incremental  ln  
ls  mkdir  mount  mv  printenv  rm  sleep  touch  

gnu ls:
build  cat  clear  coreutils  coreutils.d  cp  deps  echo  examples  incremental  ln  ls  mkdir  mount  mv  printenv  rm  sleep  touch
```

## cat
- format line numbers better:
```
8     #[arg(short, long)]
9     line_numbers: bool,
10     #[arg(required = true)]
11     paths: Vec<PathBuf>,
```
should be
```
8      #[arg(short, long)]
9      line_numbers: bool,
10     #[arg(required = true)]
11     paths: Vec<PathBuf>,
```

## cp
- support directories, probably with walkdir or some other library.
- When tryingt to copy directories without the -r/--recursive flag output a better error message

## sleep
- add supports for floats