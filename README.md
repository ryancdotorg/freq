```
Usage: freq [OPTIONS] [FILES]...

Arguments:
  [FILES]...

Options:
  -d <DIGITS>      Digits of precision [default: 3]
  -l <LIMIT>       Limit output to top N values
  -n, --number     Number lines
  -t, --tsv        Tab delimited output
  -c, --csv        Comma seperated output
  -P, --no-pct     Don't show percent
  -C, --no-cdf     Don't show CDF
  -h, --help       Print help
  -V, --version    Print version
```

```
$ tr 'A-Z ' 'a-z\n' < 2600.txt.utf-8 | tr -d ',.' | grep . | freq -l30
  34269   5.907   5.907  the
  21801   3.758   9.664  and
  16573   2.857  12.521  to
  14950   2.577  15.098  of
  13866   2.390  17.488  
  10394   1.792  19.279  a
   9574   1.650  20.929  he
   8848   1.525  22.454  in
   7949   1.370  23.825  his
   7587   1.308  25.132  that
   7302   1.259  26.391  was
   5669   0.977  27.368  with
   5348   0.922  28.290  had
   4656   0.803  29.092  it
   4587   0.791  29.883  her
   4578   0.789  30.672  not
   4503   0.776  31.448  at
   4294   0.740  32.188  him
   3913   0.674  32.863  as
   3879   0.669  33.531  on
   3663   0.631  34.163  but
   3427   0.591  34.753  for
   3316   0.572  35.325  she
   3264   0.563  35.888  i
   3163   0.545  36.433  is
   2994   0.516  36.949  you
   2757   0.475  37.424  said
   2670   0.460  37.884  from
   2640   0.455  38.339  all
   2401   0.414  38.753  were
```
