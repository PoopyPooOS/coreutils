single call binaries: 7.6M (all musl static)
multi call binary:
 - 848K musl static
 - 748K glibc dynamic
 - 196K dynamically linked crates (glibc)

## ls
- Make better use of horizontal space:
```
this ls:
build  cat  clear  coreutils  coreutils.d  cp  deps  echo  examples  incremental  ln  
ls  mkdir  mount  mv  printenv  rm  sleep  touch  

gnu ls:
build  cat  clear  coreutils  coreutils.d  cp  deps  echo  examples  incremental  ln  ls  mkdir  mount  mv  printenv  rm  sleep  touch
```

## cat
- Format line numbers better:
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
- Use 

## cp
- Support directories, probably with walkdir or some other library.

## sleep
- Add supports for floats
