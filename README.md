# Web  Benchmarker

This is a web benchmarking tool powered by rewrk. It comes with a script that can be used to run a single iteration of the benchmark and outputs a json file with the results of the benchmark. 

## Running locally

You can clone this repository and then in the root directory run `devenv shell run-bench` to test it locally. 

## Contributing

You can add new test cases by adding a new number folder in the tests directory. The folder should contain a [devenv project](https://devenv.sh/), a toml file and the folder name should be a number. 

The new folder should contain a file describing the test and should be named `test.toml`. A sample of what it should look like is below
```toml
notes = "123" # Anything important to know about the test
language = "rust" # Should be a language in the supported-languages.json file. If it does not exist, then you can add it in the same PR.
framework = "axum" # The framework used in your language
test_type = "PlainText" # The type of test. Should be one of: PlainText, Json, Database, Html
url = "http://127.0.0.1:3000/" # The URL that rewrk should hit during the test
revision = 1 # Bump this number when updates are made to the test to prioritize it more highly in the re-test schedule
```

The devenv project should contain processes that start the web framework and if it is a database test should also startup the database. Finally, the folder should be named the
next sequential folder name after the largest folder name. An example is if the latest folder is 11, then the new one should be 12.

## Machine Specs

Below is the output of fastfetch on the machine running the benchmarks

```
$ fastfetch
          ▗▄▄▄       ▗▄▄▄▄    ▄▄▄▖             someuser@bench
          ▜███▙       ▜███▙  ▟███▛             --------------
           ▜███▙       ▜███▙▟███▛              OS: NixOS XX.XX (Vicuna) x86_64
            ▜███▙       ▜██████▛               Host: SER (V1.0)
     ▟█████████████████▙ ▜████▛     ▟▙         Kernel: Linux X.X.X
    ▟███████████████████▙ ▜███▙    ▟██▙        Uptime: 9 mins
           ▄▄▄▄▖           ▜███▙  ▟███▛        Packages: 559 (nix-system)
          ▟███▛             ▜██▛ ▟███▛         Shell: bash 5.X.XX
         ▟███▛               ▜▛ ▟███▛          Terminal: /dev/pts/0
▟███████████▛                  ▟██████████▙    CPU: AMD Ryzen 7 6800U (16) @ 4.77 GHz
▜██████████▛                  ▟███████████▛    GPU: AMD Radeon 680M [Integrated]
      ▟███▛ ▟▙               ▟███▛             Memory: 678.98 MiB / 19.31 GiB (3%)
     ▟███▛ ▟██▙             ▟███▛              Swap: Disabled
    ▟███▛  ▜███▙           ▝▀▀▀▀               Disk (/): 10.43 GiB / 467.88 GiB (2%) - ext4
    ▜██▛    ▜███▙ ▜██████████████████▛         
     ▜▛     ▟████▙ ▜████████████████▛         
           ▟██████▙       ▜███▙
          ▟███▛▜███▙       ▜███▙
         ▟███▛  ▜███▙       ▜███▙
         ▝▀▀▀    ▀▀▀▀▘       ▀▀▀▘
```
