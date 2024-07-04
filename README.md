# Les(Local Echo Server)

## 简介
Les启动后会在本地启动一个http服务,主要用于调试。


## 使用说明
### help命令
```
PS D:\code\github\local-echo-server\target\release> .\les.exe --help
Usage: les.exe [OPTIONS]

Options:
  -P, --port <Port>  The http port,default port is 9345 [default: 9345]
  -h, --help         Print help
  -V, --version      Print version
```
### 直接启动(使用默认端口启动)
直接启动会在本地启动一个http服务，默认端口是9345
```
.\les.exe
```
### 使用其他端口启动
```
.\les.exe -P 9090
```
### 使用curl测试一下les的输出
```
curl -d "param1=value1&param2=value2" -X POST http://localhost:9345/data

{
  "body": "param1=value1&param2=value2",
  "headers": {
    "accept": "*/*",
    "accept-encoding": "gzip",
    "content-length": "27",
    "content-type": "application/x-www-form-urlencoded",
    "host": "localhost:9345",
    "user-agent": "curl/8.7.1"
  },
  "method": "POST",
  "origin": "127.0.0.1:49557",
  "path": "/data",
  "version": "HTTP/1.1"
}
```
```
curl -d "param1=value1&param2=value2" -H "Content-Type: application/x-www-form-urlencoded" -X POST http://localhost:9345/data
{
  "body": "param1=value1&param2=value2",
  "headers": {
    "accept": "*/*",
    "accept-encoding": "gzip",
    "content-length": "27",
    "content-type": "application/x-www-form-urlencoded",
    "host": "localhost:9345",
    "user-agent": "curl/8.7.1"
  },
  "method": "POST",
  "origin": "127.0.0.1:50160",
  "path": "/data",
  "version": "HTTP/1.1"
}
```
```
 curl -d "@mumu_boot.txt" -X POST http://localhost:9345/data
 {
  "body": "add_path_to_accelist path: C:\\Program Files\\Netease\\MuMuPlayer-12.0\\vms\\MuMuPlayer-12.0-base\\system.vdiprefetch dlls takes 7 msprint meta data: data_type: 1type_1, type: 1, size: 457147392 B, ele_num: 7544type_2, type: 2, size: 457147392 B, ele_num: 680avail phys: 24045 MBprefetch system vdi 435.969727 MB takes 151 msQt5WebEngineCore Path: C:\\Program Files\\Netease\\MuMuPlayer-12.0\\device\\\\..\\EmulatorShell\\Qt5WebEngineCore.dllPrefetchFileReader::open CreateFileW fail, err: 3prefetch webengine dll takes 0 ms",
  "headers": {
    "accept": "*/*",
    "accept-encoding": "gzip",
    "content-length": "510",
    "content-type": "application/x-www-form-urlencoded",
    "host": "localhost:9345",
    "user-agent": "curl/8.7.1"
  },
  "method": "POST",
  "origin": "127.0.0.1:50798",
  "path": "/data",
  "version": "HTTP/1.1"
}
```

### 注意
- 本程序支持windows(不包含windows7及以下)，Mac,Linux，可以直接去[release](https://github.com/lsk569937453/local-echo-server/releases)下载。
- 本程序目前不支持https,正在开发。