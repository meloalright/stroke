![Logo](https://user-images.githubusercontent.com/11075892/164912746-7923ed22-42ae-4dda-a77b-bef6fd8c0109.png)

# stroke

A MacOS command stroke tool written in Rust

一款基于 Rust 实现的命令行绘线工具


## Install

```
$ brew tap meloalright/tap 
$ brew install meloalright/tap/stroke 
```

## Usage

### ARGS: \<to.x\> \<to.y\> \<to.x\> \<to.y\>...


![output](https://user-images.githubusercontent.com/11075892/164912960-98b62f91-83c2-455d-8804-186692246edc.png)

```
$ stroke 0 0 30 60 70 40 120 120
```


### --color \<COLOR\>

![output](https://user-images.githubusercontent.com/11075892/164912978-061f9f93-cd09-4740-bb2a-6f2036eb5e17.png)

```
$ stroke 0 0 30 60 70 40 120 120 --color c200e8
```


### --output \<OUTPUT\>

```shell
$ stroke 0 0 30 60 70 40 120 120 --output mypath.png

# You can just run:
#   open mypath.png
```