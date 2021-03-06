# Investigate nom vs regex for Common Log Format.

This project compares a [regex](https://github.com/rust-lang/regex) based [Common Log Format](https://en.wikipedia.org/wiki/Common_Log_Format) parser against [nom](https://github.com/Geal/nom).

We use [ripgrep](https://github.com/BurntSushi/ripgrep) as a baseline.

```
› /usr/bin/time rg '([\da-f\.:]*) (.*) (.*) \[(.*)\] "(.*)" (\d{3}) (\d*).*' data/small_access.log -r '$1 , $2, $3 ,$4 , $5, $6, $7' > /dev/null
3.68user 0.00system 0:03.70elapsed 99%CPU
```

`data/small_access.log` is a 35M file containing 161761 Nginx access lines. Both parsers convert the response code and the response size to an integer.

One can run the benchmark with `cargo bench` but be aware that the regex parser takes a long time.

## Results

A quick run shows that the nom parser is faster than `rg` and regex. The regex parser cannot match the speed of `rg`.

```
› /usr/bin/time target/release/parse nom
0.05user 0.01system 0:00.07elapsed 100%CPU
```

```
› /usr/bin/time target/release/parse regex
9.05user 0.01system 0:09.08elapsed 99%CPU
```

These quick runs are confirmed by longer `cargo bench` runs. The results might be surprising. There is some discussion on the regular expression used here in [rust-lang/regex#389](https://github.com/rust-lang/regex/issues/389).
