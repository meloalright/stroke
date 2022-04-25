![Logo](https://user-images.githubusercontent.com/11075892/164912746-7923ed22-42ae-4dda-a77b-bef6fd8c0109.png)

# Stroke

`A MacOS command stroke tool written in Rust`

`一款基于 Rust 实现的 Mac 命令行一笔画绘线工具`


## Install

```shell
$ brew install meloalright/tap/stroke
```

## Usage

### Quick Start

![output](https://user-images.githubusercontent.com/11075892/164912960-98b62f91-83c2-455d-8804-186692246edc.png)

```
$ stroke 0 0 30 60 70 40 120 120 --view
```


### --color \<COLOR\>

![output](https://user-images.githubusercontent.com/11075892/164912978-061f9f93-cd09-4740-bb2a-6f2036eb5e17.png)

```
$ stroke 0 0 30 60 70 40 120 120 --color c200e8
```


### --output \<OUTPUT\>

```shell
$ stroke 0 0 30 60 70 40 120 120 --output mypath.png
```


### --view

```shell
$ stroke 0 0 30 60 70 40 120 120 --view
```

### Stroke the Graphic with one line

![star](https://user-images.githubusercontent.com/11075892/164914991-2d8519cc-c1c9-4b6e-ab8d-ed8bc6e1c130.png)

```shell
$ stroke 0 40 120 40 20 120 60 0 100 120 0 40 60 0 120 40 100 120 20 120 0 40 --view
```

## Development

```shell
$ git clone https://github.com/meloalright/stroke
```

```shell
$ cargo build

$ ./target/debug/stroke -h
```


```shell
$ cargo build --release
```

## License   
   
[MIT](https://opensource.org/licenses/MIT)   
